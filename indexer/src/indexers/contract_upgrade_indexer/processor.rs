use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_indexer_transaction_stream::{TransactionStream, TransactionStreamConfig},
    builder::ProcessorBuilder,
    common_steps::TransactionStreamStep,
    traits::IntoRunnableStep,
};

use super::{extractor::Extractor, storer::Storer};
use crate::{
    config::indexer_processor_config::{CustomConfig, IndexerProcessorConfig},
    utils::{
        chain_id::check_or_update_chain_id, database_connection::new_db_pool,
        database_utils::ArcDbPool, latest_processed_version_tracker::LatestVersionProcessedTracker,
        starting_version::get_starting_version,
    },
};

pub struct ContractUpgradeProcessor {
    pub config: IndexerProcessorConfig,
    pub db_pool: ArcDbPool,
}

impl ContractUpgradeProcessor {
    pub async fn new(config: IndexerProcessorConfig) -> Result<Self> {
        let conn_pool = new_db_pool(
            &config.db_config.postgres_connection_string,
            config.db_config.db_pool_size,
        )
        .await;

        Ok(Self {
            config,
            db_pool: conn_pool,
        })
    }

    pub async fn run_processor(self) -> Result<()> {
        // Merge the starting version from config and the latest processed version from the DB
        let starting_version = get_starting_version(&self.config, self.db_pool.clone()).await?;

        tracing::info!(
            "Starting events processor with starting version: {:?}",
            starting_version
        );

        // Check and update the ledger chain id to ensure we're indexing the correct chain
        let grpc_chain_id = TransactionStream::new(self.config.transaction_stream_config.clone())
            .await?
            .get_chain_id()
            .await?;
        check_or_update_chain_id(grpc_chain_id as i64, self.db_pool.clone()).await?;

        // Define processor steps
        let transaction_stream = TransactionStreamStep::new(TransactionStreamConfig {
            starting_version: Some(starting_version),
            ..self.config.transaction_stream_config
        })
        .await?;
        let events_extractor = Extractor::new(match self.config.custom_config {
            CustomConfig::ContractUpgradeIndexer(contract_addresses) => contract_addresses,
            _ => {
                return Err(anyhow::anyhow!("Invalid custom config"));
            }
        });
        let events_storer = Storer::new(self.db_pool.clone());
        let version_tracker = LatestVersionProcessedTracker::new(
            self.config.db_config,
            starting_version,
            self.config.processor_config.name().to_string(),
        )
        .await?;

        // Connect processor steps together
        let (_, buffer_receiver) = ProcessorBuilder::new_with_inputless_first_step(
            transaction_stream.into_runnable_step(),
        )
        .connect_to(events_extractor.into_runnable_step(), 10)
        .connect_to(events_storer.into_runnable_step(), 10)
        .connect_to(version_tracker.into_runnable_step(), 10)
        .end_and_return_output_receiver(10);

        // (Optional) Parse the results
        loop {
            match buffer_receiver.recv().await {
                Ok(txn_context) => {
                    if txn_context.data.events.is_empty() && txn_context.data.changes.is_empty() {
                        continue;
                    }
                    tracing::info!(
                        "Finished processing events from versions [{:?}, {:?}]",
                        txn_context.metadata.start_version,
                        txn_context.metadata.end_version,
                    );
                }
                Err(_) => {
                    tracing::error!("Channel is closed");
                    return Ok(());
                }
            }
        }
    }
}
