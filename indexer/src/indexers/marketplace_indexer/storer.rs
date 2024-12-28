use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    traits::{async_step::AsyncRunType, AsyncStep, NamedStep, Processable},
    types::transaction_context::TransactionContext,
    utils::errors::ProcessorError,
};
use async_trait::async_trait;

use super::{
    extractor::{ContractEvent, TransactionContextData},
    storers::{
        upgrade_module_change_storer::process_upgrade_module_changes,
        upgrade_package_change_storer::process_upgrade_package_changes,
    },
};
use crate::utils::database_utils::ArcDbPool;

/// Storer is a step that inserts events in the database.
pub struct Storer
where
    Self: Sized + Send + 'static,
{
    pool: ArcDbPool,
}

impl AsyncStep for Storer {}

impl NamedStep for Storer {
    fn name(&self) -> String {
        "Storer".to_string()
    }
}

impl Storer {
    pub fn new(pool: ArcDbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Processable for Storer {
    type Input = TransactionContextData;
    type Output = TransactionContextData;
    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        transaction_context_data: TransactionContext<TransactionContextData>,
    ) -> Result<Option<TransactionContext<TransactionContextData>>, ProcessorError> {
        let per_table_chunk_sizes: AHashMap<String, usize> = AHashMap::new();
        let data = transaction_context_data.data.clone();
        let (create_events, update_events) = data.events.into_iter().fold(
            (
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ),
            |(
                mut bid_placed_events,
                mut bid_filled_events,
                mut bid_cancelled_events,
                mut ask_placed_events,
                mut ask_filled_events,
                mut ask_cancelled_events,
                mut collection_bid_placed_events,
                mut collection_bid_filled_events,
                mut collection_bid_cancelled_events,
            ),
             event| {
                match event {
                    ContractEvent::BidOrderPlacedEvent(nft_bid) => bid_placed_events.push(nft_bid),
                    ContractEvent::BidOrderFilledEvent(nft_bid) => bid_filled_events.push(nft_bid),
                    ContractEvent::BidOrderCancelledEvent(nft_bid) => {
                        bid_cancelled_events.push(nft_bid)
                    }
                    ContractEvent::AskOrderPlacedEvent(nft_ask) => ask_placed_events.push(nft_ask),
                    ContractEvent::AskOrderFilledEvent(nft_ask) => ask_filled_events.push(nft_ask),
                    ContractEvent::AskOrderCancelledEvent(nft_ask) => {
                        ask_cancelled_events.push(nft_ask)
                    }
                    ContractEvent::CollectionBidOrderPlacedEvent(collection_bid) => {
                        collection_bid_placed_events.push(collection_bid)
                    }
                    ContractEvent::CollectionBidOrderFilledEvent((
                        collection_bid,
                        filled_collection_bid,
                    )) => {
                        collection_bid_filled_events.push((collection_bid, filled_collection_bid))
                    }
                    ContractEvent::CollectionBidOrderCancelledEvent(collection_bid) => {
                        collection_bid_cancelled_events.push(collection_bid)
                    }
                }
                (
                    bid_placed_events,
                    bid_filled_events,
                    bid_cancelled_events,
                    ask_placed_events,
                    ask_filled_events,
                    ask_cancelled_events,
                    collection_bid_placed_events,
                    collection_bid_filled_events,
                    collection_bid_cancelled_events,
                )
            },
        );

        process_create_message_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            create_events,
        )
        .await?;

        process_update_message_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            update_events,
        )
        .await?;

        Ok(Some(transaction_context_data))
    }
}
