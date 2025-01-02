use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{nft_bids::NftBid, shared::OrderStatus},
    schema::nft_bids,
    utils::{
        database_connection::get_db_connection,
        database_execution::handle_db_execution,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_sql(
    conn: &mut AsyncPgConnection,
    items_to_insert: Vec<NftBid>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let sql = insert_into(nft_bids::table)
                .values(items_to_insert.clone())
                .on_conflict(nft_bids::bid_obj_addr)
                .do_update()
                .set((
                    nft_bids::bid_obj_addr.eq(nft_bids::bid_obj_addr),
                    nft_bids::nft_id.eq(nft_bids::nft_id),
                    nft_bids::nft_name.eq(nft_bids::nft_name),
                    nft_bids::collection_addr.eq(nft_bids::collection_addr),
                    nft_bids::collection_creator_addr.eq(nft_bids::collection_creator_addr),
                    nft_bids::collection_name.eq(nft_bids::collection_name),
                    nft_bids::nft_standard.eq(nft_bids::nft_standard),
                    nft_bids::marketplace_addr.eq(nft_bids::marketplace_addr),
                    nft_bids::buyer_addr.eq(nft_bids::buyer_addr),
                    nft_bids::price.eq(nft_bids::price),
                    nft_bids::royalties.eq(nft_bids::royalties),
                    nft_bids::commission.eq(nft_bids::commission),
                    nft_bids::payment_token.eq(nft_bids::payment_token),
                    nft_bids::payment_token_type.eq(nft_bids::payment_token_type),
                    nft_bids::order_placed_timestamp.eq(nft_bids::order_placed_timestamp),
                    nft_bids::order_placed_tx_version.eq(nft_bids::order_placed_tx_version),
                    nft_bids::order_placed_event_idx.eq(nft_bids::order_placed_event_idx),
                    nft_bids::order_filled_timestamp.eq(excluded(nft_bids::order_filled_timestamp)),
                    nft_bids::order_filled_tx_version.eq(excluded(nft_bids::order_filled_tx_version)),
                    nft_bids::order_filled_event_idx.eq(excluded(nft_bids::order_filled_event_idx)),
                    nft_bids::order_cancelled_timestamp.eq(nft_bids::order_cancelled_timestamp),
                    nft_bids::order_cancelled_tx_version.eq(nft_bids::order_cancelled_tx_version),
                    nft_bids::order_cancelled_event_idx.eq(nft_bids::order_cancelled_event_idx),
                    nft_bids::order_status.eq(excluded(nft_bids::order_status)),
                ))
                .filter(
                    // Update only if previous status is open
                    nft_bids::order_status.eq(OrderStatus::Open as i32),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_bid_filled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<NftBid>,
) -> Result<(), ProcessorError> {
    let chunk_size = get_config_table_chunk_size::<NftBid>("nft_bids", &per_table_chunk_sizes);
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
