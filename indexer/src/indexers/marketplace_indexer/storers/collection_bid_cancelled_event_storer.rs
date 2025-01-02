use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{collection_bids::CollectionBid, shared::OrderStatus},
    schema::collection_bids,
    utils::{
        database_connection::get_db_connection,
        database_execution::handle_db_execution,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_sql(
    conn: &mut AsyncPgConnection,
    items_to_insert: Vec<CollectionBid>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let sql = insert_into(collection_bids::table)
                .values(items_to_insert.clone())
                .on_conflict(collection_bids::bid_obj_addr)
                .do_update()
                .set((
                    collection_bids::bid_obj_addr.eq(collection_bids::bid_obj_addr),
                    collection_bids::collection_addr.eq(collection_bids::collection_addr),
                    collection_bids::collection_creator_addr
                        .eq(collection_bids::collection_creator_addr),
                    collection_bids::collection_name.eq(collection_bids::collection_name),
                    collection_bids::nft_standard.eq(collection_bids::nft_standard),
                    collection_bids::marketplace_addr.eq(collection_bids::marketplace_addr),
                    collection_bids::buyer_addr.eq(collection_bids::buyer_addr),
                    collection_bids::price.eq(collection_bids::price),
                    collection_bids::payment_token.eq(collection_bids::payment_token),
                    collection_bids::payment_token_type.eq(collection_bids::payment_token_type),
                    collection_bids::order_placed_timestamp
                        .eq(collection_bids::order_placed_timestamp),
                    collection_bids::order_placed_tx_version
                        .eq(collection_bids::order_placed_tx_version),
                    collection_bids::order_placed_event_idx
                        .eq(collection_bids::order_placed_event_idx),
                    collection_bids::latest_order_filled_event_idx
                        .eq(collection_bids::latest_order_filled_event_idx),
                    collection_bids::latest_order_filled_timestamp
                        .eq(collection_bids::latest_order_filled_timestamp),
                    collection_bids::latest_order_filled_tx_version
                        .eq(collection_bids::latest_order_filled_tx_version),
                    collection_bids::order_cancelled_timestamp
                        .eq(excluded(collection_bids::order_cancelled_timestamp)),
                    collection_bids::order_cancelled_tx_version
                        .eq(excluded(collection_bids::order_cancelled_tx_version)),
                    collection_bids::order_cancelled_event_idx
                        .eq(excluded(collection_bids::order_cancelled_event_idx)),
                    collection_bids::order_status.eq(excluded(collection_bids::order_status)),
                ))
                .filter(
                    // Update only if previous status is open
                    collection_bids::order_status.eq(OrderStatus::Open as i32),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_bid_cancelled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<CollectionBid>,
) -> Result<(), ProcessorError> {
    let chunk_size =
        get_config_table_chunk_size::<CollectionBid>("collection_bids", &per_table_chunk_sizes);
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
