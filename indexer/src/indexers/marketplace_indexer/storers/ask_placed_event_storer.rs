use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{insert_into, QueryResult};
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
                .do_nothing();
            sql.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_ask_placed_events(
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
