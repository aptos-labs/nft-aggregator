use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, BoolExpressionMethods,
    ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{activities::Activity, nft_asks::NftAsk},
    schema::{activities, nft_asks},
    utils::{
        database_connection::get_db_connection,
        database_execution::handle_db_execution,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_sql(
    conn: &mut AsyncPgConnection,
    items_to_insert: Vec<(NftAsk, Activity)>,
) -> QueryResult<()> {
    let (asks, activities): (Vec<NftAsk>, Vec<Activity>) = items_to_insert.into_iter().unzip();

    conn.transaction(|conn| {
        Box::pin(async move {
            let insert_asks = insert_into(nft_asks::table)
                .values(asks.clone())
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
            insert_asks.execute(conn).await?;

            let insert_activities = insert_into(activities::table)
                .values(activities)
                .on_conflict((
                    activities::activity_tx_version,
                    activities::activity_event_idx,
                ))
                .do_nothing();
            insert_activities.execute(conn).await?;

            Ok(())
        })
    })
    .await
}

pub async fn process_ask_filled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<(NftAsk, Activity)>,
) -> Result<(), ProcessorError> {
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

    match handle_db_execution(tasks).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "error writing ask filled events to db: {:?} with error: {:?}",
                events, e
            );
            Err(e)
        }
    }
}
