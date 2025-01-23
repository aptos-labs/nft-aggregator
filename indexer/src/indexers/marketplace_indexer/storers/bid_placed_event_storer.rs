use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, BoolExpressionMethods,
    ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::nft_bids::NftBid,
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
                    nft_bids::buyer_addr.eq(excluded(nft_bids::buyer_addr)),
                    nft_bids::price.eq(excluded(nft_bids::price)),
                    nft_bids::order_placed_timestamp.eq(excluded(nft_bids::order_placed_timestamp)),
                    nft_bids::order_placed_tx_version
                        .eq(excluded(nft_bids::order_placed_tx_version)),
                    nft_bids::order_placed_event_idx.eq(excluded(nft_bids::order_placed_event_idx)),
                    nft_bids::order_status.eq(excluded(nft_bids::order_status)),
                ))
                .filter(
                    // Update only if tx version is greater than the existing one
                    // or if the tx version is the same but the event index is greater
                    nft_bids::order_placed_tx_version
                        .lt(excluded(nft_bids::order_placed_tx_version))
                        .or(nft_bids::order_placed_tx_version
                            .eq(excluded(nft_bids::order_placed_tx_version))
                            .and(
                                nft_bids::order_placed_event_idx
                                    .lt(excluded(nft_bids::order_placed_event_idx)),
                            )),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_bid_placed_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<NftBid>,
) -> Result<(), ProcessorError> {
    // when order is updated, contract also emits an order placed event, so we need to deduplicate the events
    let mut unique_events_map: AHashMap<String, NftBid> = AHashMap::new();
    for event in events {
        if let Some(existing_event) = unique_events_map.get_mut(&event.bid_obj_addr) {
            if event.order_placed_tx_version > existing_event.order_placed_tx_version
                || event.order_placed_tx_version == existing_event.order_placed_tx_version
                    && event.order_placed_event_idx > existing_event.order_placed_event_idx
            {
                *existing_event = event;
            }
        } else {
            unique_events_map.insert(event.bid_obj_addr.clone(), event);
        }
    }
    let unique_events = unique_events_map
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>();

    let chunk_size = get_config_table_chunk_size::<NftBid>("nft_bids", &per_table_chunk_sizes);
    let tasks = unique_events
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            tokio::spawn(async move {
                let conn = &mut get_db_connection(&pool).await.expect(
                    "Failed to get connection from pool while processing bid placed events",
                );
                execute_sql(conn, items).await
            })
        })
        .collect::<Vec<_>>();

    match handle_db_execution(tasks).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "error writing bid placed events to db: {:?} with error: {:?}",
                unique_events, e
            );
            Err(e)
        }
    }
}
