use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    debug_query,
    pg::Pg,
    query_builder::{QueryFragment, QueryId},
    result::Error as DieselError,
    QueryResult,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use futures_util::future;

pub async fn execute_with_better_error<U>(
    conn: &mut AsyncPgConnection,
    queries: Vec<U>,
) -> QueryResult<()>
where
    U: QueryFragment<Pg> + QueryId + Send,
{
    let debug_query = queries
        .iter()
        .map(|q| debug_query::<Pg, _>(q).to_string())
        .collect::<Vec<_>>();
    let res = conn
        .transaction(|conn| {
            Box::pin(async move {
                for q in queries {
                    q.execute(conn).await?;
                }
                Ok(())
            })
        })
        .await;
    if let Err(ref e) = res {
        tracing::error!("Error running query: {:?}\n{:?}", e, debug_query);
    }
    res
}
pub async fn handle_db_execution(
    tasks: Vec<tokio::task::JoinHandle<Result<(), DieselError>>>,
) -> Result<(), ProcessorError> {
    let results = future::try_join_all(tasks)
        .await
        .expect("Task panicked executing in chunks");
    for res in results {
        res.map_err(|e| {
            tracing::warn!("Error running query: {:?}", e);
            ProcessorError::ProcessError {
                message: e.to_string(),
            }
        })?;
    }
    Ok(())
}
