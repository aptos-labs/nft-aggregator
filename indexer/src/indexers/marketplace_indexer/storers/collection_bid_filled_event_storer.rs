use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, BoolExpressionMethods,
    ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::collection_bids::{CollectionBid, FilledCollectionBid},
    schema::{collection_bids, filled_collection_bids},
    utils::{
        database_connection::get_db_connection,
        database_execution::handle_db_execution,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_sql(
    conn: &mut AsyncPgConnection,
    unique_collection_bids: Vec<CollectionBid>,
    all_filled_collection_bids: Vec<FilledCollectionBid>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let sql = insert_into(collection_bids::table)
                .values(unique_collection_bids.clone())
                .on_conflict(collection_bids::bid_obj_addr)
                .do_update()
                .set((
                    collection_bids::latest_order_filled_tx_version
                        .eq(excluded(collection_bids::latest_order_filled_tx_version)),
                    collection_bids::latest_order_filled_event_idx
                        .eq(excluded(collection_bids::latest_order_filled_event_idx)),
                    collection_bids::latest_order_filled_timestamp
                        .eq(excluded(collection_bids::latest_order_filled_timestamp)),
                ))
                .filter(
                    // Update only if tx version is greater than the existing one
                    // or if the tx version is the same but the event index is greater
                    collection_bids::latest_order_filled_tx_version
                        .lt(excluded(collection_bids::latest_order_filled_tx_version))
                        .or(collection_bids::latest_order_filled_tx_version
                            .eq(excluded(collection_bids::latest_order_filled_tx_version))
                            .and(
                                collection_bids::latest_order_filled_event_idx
                                    .lt(excluded(collection_bids::latest_order_filled_event_idx)),
                            )),
                );
            sql.execute(conn).await?;

            let sql = insert_into(filled_collection_bids::table)
                .values(all_filled_collection_bids.clone())
                .on_conflict((
                    filled_collection_bids::bid_obj_addr,
                    filled_collection_bids::nft_id,
                    filled_collection_bids::nft_name,
                ))
                .do_update()
                .set((
                    filled_collection_bids::seller_addr
                        .eq(excluded(filled_collection_bids::seller_addr)),
                    filled_collection_bids::royalties
                        .eq(excluded(filled_collection_bids::royalties)),
                    filled_collection_bids::commission
                        .eq(excluded(filled_collection_bids::commission)),
                    filled_collection_bids::price.eq(excluded(filled_collection_bids::price)),
                    filled_collection_bids::order_filled_timestamp
                        .eq(excluded(filled_collection_bids::order_filled_timestamp)),
                    filled_collection_bids::order_filled_tx_version
                        .eq(excluded(filled_collection_bids::order_filled_tx_version)),
                    filled_collection_bids::order_filled_event_idx
                        .eq(excluded(filled_collection_bids::order_filled_event_idx)),
                ))
                .filter(
                    filled_collection_bids::order_filled_tx_version
                        .lt(excluded(filled_collection_bids::order_filled_tx_version))
                        .or(filled_collection_bids::order_filled_tx_version
                            .eq(excluded(filled_collection_bids::order_filled_tx_version))
                            .and(
                                filled_collection_bids::order_filled_event_idx
                                    .lt(excluded(filled_collection_bids::order_filled_event_idx)),
                            )),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_collection_bid_filled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<(CollectionBid, FilledCollectionBid)>,
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
                    "Failed to get connection from pool while processing collection bid filled events",
                );

                let mut all_filled_collection_bids = vec![];
                let mut unique_collection_bids: AHashMap<String, CollectionBid> = AHashMap::new();

                for (curr_collection_bid, curr_filled_collection_bid) in items.clone() {
                    // if not exist, insert, otherwise use the one with greater tx version and event index
                    let existing_collection_bid = unique_collection_bids
                        .get(&curr_collection_bid.bid_obj_addr)
                        .cloned();
                    match existing_collection_bid {
                        Some(existing_collection_bid) => {
                            if curr_collection_bid.order_placed_tx_version
                                > existing_collection_bid.order_placed_tx_version
                                || (curr_collection_bid.order_placed_tx_version
                                    == existing_collection_bid.order_placed_tx_version
                                    && curr_collection_bid.order_placed_event_idx
                                        > existing_collection_bid.order_placed_event_idx)
                            {
                                unique_collection_bids.insert(
                                    curr_collection_bid.bid_obj_addr.clone(),
                                    curr_collection_bid.clone(),
                                );
                            }
                        }
                        None => {
                            unique_collection_bids.insert(
                                curr_collection_bid.bid_obj_addr.clone(),
                                curr_collection_bid.clone(),
                            );
                        }
                    }

                    all_filled_collection_bids.push(curr_filled_collection_bid);
                }

                execute_sql(
                    conn,
                    unique_collection_bids.values().cloned().collect(),
                    all_filled_collection_bids,
                )
                .await
            })
        })
        .collect::<Vec<_>>();

    handle_db_execution(tasks).await
}
