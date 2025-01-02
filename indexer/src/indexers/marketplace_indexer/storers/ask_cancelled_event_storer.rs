use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{nft_asks::NftAsk, shared::OrderStatus},
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
                    nft_asks::ask_obj_addr.eq(nft_asks::ask_obj_addr),
                    nft_asks::nft_id.eq(nft_asks::nft_id),
                    nft_asks::nft_name.eq(nft_asks::nft_name),
                    nft_asks::collection_addr.eq(nft_asks::collection_addr),
                    nft_asks::collection_creator_addr.eq(nft_asks::collection_creator_addr),
                    nft_asks::collection_name.eq(nft_asks::collection_name),
                    nft_asks::nft_standard.eq(nft_asks::nft_standard),
                    nft_asks::marketplace_addr.eq(nft_asks::marketplace_addr),
                    nft_asks::seller_addr.eq(nft_asks::seller_addr),
                    nft_asks::price.eq(nft_asks::price),
                    nft_asks::royalties.eq(nft_asks::royalties),
                    nft_asks::commission.eq(nft_asks::commission),
                    nft_asks::payment_token.eq(nft_asks::payment_token),
                    nft_asks::payment_token_type.eq(nft_asks::payment_token_type),
                    nft_asks::order_placed_timestamp.eq(nft_asks::order_placed_timestamp),
                    nft_asks::order_placed_tx_version.eq(nft_asks::order_placed_tx_version),
                    nft_asks::order_placed_event_idx.eq(nft_asks::order_placed_event_idx),
                    nft_asks::order_filled_timestamp.eq(nft_asks::order_filled_timestamp),
                    nft_asks::order_filled_tx_version.eq(nft_asks::order_filled_tx_version),
                    nft_asks::order_filled_event_idx.eq(nft_asks::order_filled_event_idx),
                    nft_asks::order_cancelled_timestamp
                        .eq(excluded(nft_asks::order_cancelled_timestamp)),
                    nft_asks::order_cancelled_tx_version
                        .eq(excluded(nft_asks::order_cancelled_tx_version)),
                    nft_asks::order_cancelled_event_idx
                        .eq(excluded(nft_asks::order_cancelled_event_idx)),
                    nft_asks::order_status.eq(excluded(nft_asks::order_status)),
                ))
                .filter(
                    // Update only if previous status is open
                    nft_asks::order_status.eq(OrderStatus::Open as i32),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_ask_cancelled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<NftAsk>,
) -> Result<(), ProcessorError> {
    let chunk_size = get_config_table_chunk_size::<NftAsk>("nft_asks", &per_table_chunk_sizes);
    let tasks = events
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            tokio::spawn(async move {
                let conn = &mut get_db_connection(&pool).await.expect(
                    "Failed to get connection from pool while processing create message events",
                );
                execute_sql(conn, items).await
            })
        })
        .collect::<Vec<_>>();

    handle_db_execution(tasks).await
}
