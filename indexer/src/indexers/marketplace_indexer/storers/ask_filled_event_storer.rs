use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, BoolExpressionMethods,
    ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::nft_asks::NftAsk,
    schema::nft_asks,
    utils::{
        database_connection::get_db_connection,
        database_execution::handle_db_execution,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_sql(
    conn: &mut AsyncPgConnection,
    items_to_insert: Vec<NftAsk>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let sql = insert_into(nft_asks::table)
                .values(items_to_insert.clone())
                .on_conflict(nft_asks::ask_obj_addr)
                .do_update()
                .set((
                    nft_asks::buyer_addr.eq(excluded(nft_asks::buyer_addr)),
                    nft_asks::royalties.eq(excluded(nft_asks::royalties)),
                    nft_asks::commission.eq(excluded(nft_asks::commission)),
                    nft_asks::price.eq(excluded(nft_asks::price)),
                    nft_asks::order_filled_timestamp.eq(excluded(nft_asks::order_filled_timestamp)),
                    nft_asks::order_filled_tx_version
                        .eq(excluded(nft_asks::order_filled_tx_version)),
                    nft_asks::order_filled_event_idx.eq(excluded(nft_asks::order_filled_event_idx)),
                    nft_asks::order_status.eq(excluded(nft_asks::order_status)),
                ))
                .filter(
                    // Update only if tx version is greater than the existing one
                    // or if the tx version is the same but the event index is greater
                    nft_asks::order_filled_tx_version
                        .lt(excluded(nft_asks::order_filled_tx_version))
                        .or(nft_asks::order_filled_tx_version
                            .eq(excluded(nft_asks::order_filled_tx_version))
                            .and(
                                nft_asks::order_filled_event_idx
                                    .lt(excluded(nft_asks::order_filled_event_idx)),
                            )),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_ask_filled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<NftAsk>,
) -> Result<(), ProcessorError> {
    let mut unique_events: AHashMap<String, NftAsk> = AHashMap::new();
    for event in events.clone() {
        if let Some(existing_event) = unique_events.get_mut(&event.ask_obj_addr) {
            panic!(
                "unexpected duplicate ask filled event at different tx, {:?}, {:?}",
                existing_event, event
            )
        } else {
            unique_events.insert(event.ask_obj_addr.clone(), event);
        }
    }

    println!("writing ask filled events to db: {:?}", events);

    let chunk_size = get_config_table_chunk_size::<NftAsk>("nft_asks", &per_table_chunk_sizes);
    let tasks = events
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            tokio::spawn(async move {
                let conn = &mut get_db_connection(&pool).await.expect(
                    "Failed to get connection from pool while processing ask filled events",
                );
                execute_sql(conn, items).await
            })
        })
        .collect::<Vec<_>>();

    handle_db_execution(tasks).await
}
