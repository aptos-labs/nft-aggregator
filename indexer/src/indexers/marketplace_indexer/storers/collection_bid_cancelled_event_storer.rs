use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    insert_into, query_dsl::methods::FilterDsl, upsert::excluded, BoolExpressionMethods,
    ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::collection_bids::CollectionBid,
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
                    collection_bids::price.eq(excluded(collection_bids::price)),
                    collection_bids::order_cancelled_timestamp
                        .eq(excluded(collection_bids::order_cancelled_timestamp)),
                    collection_bids::order_cancelled_tx_version
                        .eq(excluded(collection_bids::order_cancelled_tx_version)),
                    collection_bids::order_cancelled_event_idx
                        .eq(excluded(collection_bids::order_cancelled_event_idx)),
                    collection_bids::order_status.eq(excluded(collection_bids::order_status)),
                ))
                .filter(
                    // Update only if tx version is greater than the existing one
                    // or if the tx version is the same but the event index is greater
                    collection_bids::order_cancelled_tx_version
                        .lt(excluded(collection_bids::order_cancelled_tx_version))
                        .or(collection_bids::order_cancelled_tx_version
                            .eq(excluded(collection_bids::order_cancelled_tx_version))
                            .and(
                                collection_bids::order_cancelled_event_idx
                                    .lt(excluded(collection_bids::order_cancelled_event_idx)),
                            )),
                );
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_collection_bid_cancelled_events(
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
                    "Failed to get connection from pool while processing collection bid cancelled events",
                );
                execute_sql(conn, items).await
            })
        })
        .collect::<Vec<_>>();

    match handle_db_execution(tasks).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "error writing collection bid cancelled events to db: {:?} with error: {:?}",
                events, e
            );
            Err(e)
        }
    }
}
