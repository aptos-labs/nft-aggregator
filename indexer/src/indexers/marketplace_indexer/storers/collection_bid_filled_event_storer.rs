use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, BoolExpressionMethods,
    ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{
        activities::Activity, collection_bids::CollectionBid,
        filled_collection_bids::FilledCollectionBid,
    },
    schema::{activities, collection_bids, filled_collection_bids},
    utils::{
        database_connection::get_db_connection,
        database_execution::handle_db_execution,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_sql(
    conn: &mut AsyncPgConnection,
    data: Vec<(CollectionBid, Vec<(FilledCollectionBid, Activity)>)>,
) -> QueryResult<()> {
    let (collection_bids, filled_collection_bids, activities): (
        Vec<CollectionBid>,
        Vec<FilledCollectionBid>,
        Vec<Activity>,
    ) = data.into_iter().fold(
        (vec![], vec![], vec![]),
        |mut acc, (collection_bid, filled_collection_bid_activity)| {
            let (filled_collection_bids, activities): (Vec<FilledCollectionBid>, Vec<Activity>) =
                filled_collection_bid_activity.into_iter().unzip();
            acc.0.push(collection_bid);
            acc.1.extend(filled_collection_bids);
            acc.2.extend(activities);
            acc
        },
    );

    conn.transaction(|conn| {
        Box::pin(async move {
            let insert_bids = insert_into(collection_bids::table)
                .values(collection_bids.clone())
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
            insert_bids.execute(conn).await?;

            let insert_filled_bids = insert_into(filled_collection_bids::table)
                .values(filled_collection_bids.clone())
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
            insert_filled_bids.execute(conn).await?;

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

pub async fn process_collection_bid_filled_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    events: Vec<(CollectionBid, FilledCollectionBid, Activity)>,
) -> Result<(), ProcessorError> {
    let mut collection_bids_map: AHashMap<
        String,
        (CollectionBid, Vec<(FilledCollectionBid, Activity)>),
    > = AHashMap::new();

    for (curr_collection_bid, curr_filled_collection_bid, activity) in events.clone() {
        // if not exist, insert, otherwise use the one with greater tx version and event index
        let existing_collection_bid = collection_bids_map
            .get(&curr_collection_bid.bid_obj_addr)
            .cloned();
        match existing_collection_bid {
            Some(mut existing_collection_bid) => {
                existing_collection_bid
                    .1
                    .push((curr_filled_collection_bid, activity));
                if curr_collection_bid.order_placed_tx_version
                    > existing_collection_bid.0.order_placed_tx_version
                    || (curr_collection_bid.order_placed_tx_version
                        == existing_collection_bid.0.order_placed_tx_version
                        && curr_collection_bid.order_placed_event_idx
                            > existing_collection_bid.0.order_placed_event_idx)
                {
                    collection_bids_map.insert(
                        curr_collection_bid.bid_obj_addr.clone(),
                        (curr_collection_bid.clone(), existing_collection_bid.1),
                    );
                }
            }
            None => {
                collection_bids_map.insert(
                    curr_collection_bid.bid_obj_addr.clone(),
                    (
                        curr_collection_bid.clone(),
                        vec![(curr_filled_collection_bid, activity)],
                    ),
                );
            }
        }
    }

    let collection_bids = collection_bids_map.values().cloned().collect::<Vec<_>>();

    let chunk_size =
        get_config_table_chunk_size::<CollectionBid>("collection_bids", &per_table_chunk_sizes);
    let tasks = collection_bids
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            tokio::spawn(async move {
                let conn = &mut get_db_connection(&pool).await.expect(
                    "Failed to get connection from pool while processing collection bid filled events",
                );
                execute_sql(
                    conn,
                    items
                )
                .await
            })
        })
        .collect::<Vec<_>>();

    match handle_db_execution(tasks).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "error writing collection bid filled events to db: {:?} with error: {:?}",
                collection_bids, e
            );
            Err(e)
        }
    }
}
