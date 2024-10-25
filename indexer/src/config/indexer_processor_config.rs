use anyhow::Result;
use aptos_indexer_processor_sdk::aptos_indexer_transaction_stream::TransactionStreamConfig;
use aptos_indexer_processor_sdk_server_framework::RunnableConfig;
use serde::{Deserialize, Serialize};

use super::processor_config::ProcessorConfig;
use crate::indexers::contract_upgrade_indexer::processor::ContractUpgradeProcessor;

pub const QUERY_DEFAULT_RETRIES: u32 = 5;
pub const QUERY_DEFAULT_RETRY_DELAY_MS: u64 = 500;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexerProcessorConfig {
    pub processor_config: ProcessorConfig,
    pub transaction_stream_config: TransactionStreamConfig,
    pub db_config: DbConfig,
    pub custom_config: CustomConfig,
}

#[async_trait::async_trait]
impl RunnableConfig for IndexerProcessorConfig {
    async fn run(&self) -> Result<()> {
        match self.processor_config {
            ProcessorConfig::ContractUpgradeIndexer => {
                let events_processor = ContractUpgradeProcessor::new(self.clone()).await?;
                events_processor.run_processor().await
            }
            ProcessorConfig::TradeportIndexer => {
                return Err(anyhow::anyhow!("TradeportIndexer not implemented"));
            }
        }
    }

    fn get_server_name(&self) -> String {
        // Get the part before the first _ and trim to 12 characters.
        let before_underscore = self
            .processor_config
            .name()
            .split('_')
            .next()
            .unwrap_or("unknown");
        before_underscore[..before_underscore.len().min(12)].to_string()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DbConfig {
    pub postgres_connection_string: String,
    // Size of the pool for writes/reads to the DB. Limits maximum number of queries in flight
    #[serde(default = "DbConfig::default_db_pool_size")]
    pub db_pool_size: u32,
}

impl DbConfig {
    pub const fn default_db_pool_size() -> u32 {
        50
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomConfig {
    ContractUpgradeIndexer(Vec<String>),
    MarketplaceIndexer(String),
}
