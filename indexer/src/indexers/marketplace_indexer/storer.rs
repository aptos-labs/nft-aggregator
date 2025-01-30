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
        ask_cancelled_event_storer::process_ask_cancelled_events,
        ask_filled_event_storer::process_ask_filled_events,
        ask_placed_event_storer::process_ask_placed_events,
        bid_cancelled_event_storer::process_bid_cancelled_events,
        bid_filled_event_storer::process_bid_filled_events,
        bid_placed_event_storer::process_bid_placed_events,
        collection_bid_cancelled_event_storer::process_collection_bid_cancelled_events,
        collection_bid_filled_event_storer::process_collection_bid_filled_events,
        collection_bid_placed_event_storer::process_collection_bid_placed_events,
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
        let (
            bid_placed_events,
            bid_filled_events,
            bid_cancelled_events,
            ask_placed_events,
            ask_filled_events,
            ask_cancelled_events,
            collection_bid_placed_events,
            collection_bid_filled_events,
            collection_bid_cancelled_events,
        ) = data.events.into_iter().fold(
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
                    ContractEvent::BidPlacedEvent(nft_bid) => bid_placed_events.push(nft_bid),
                    ContractEvent::BidFilledEvent(nft_bid) => bid_filled_events.push(nft_bid),
                    ContractEvent::BidCancelledEvent(nft_bid) => bid_cancelled_events.push(nft_bid),
                    ContractEvent::AskPlacedEvent(nft_ask) => ask_placed_events.push(nft_ask),
                    ContractEvent::AskFilledEvent(nft_ask) => ask_filled_events.push(nft_ask),
                    ContractEvent::AskCancelledEvent(nft_ask) => ask_cancelled_events.push(nft_ask),
                    ContractEvent::CollectionBidPlacedEvent(collection_bid) => {
                        collection_bid_placed_events.push(collection_bid)
                    }
                    ContractEvent::CollectionBidFilledEvent(filled_collection_bid) => {
                        collection_bid_filled_events.push(filled_collection_bid)
                    }
                    ContractEvent::CollectionBidCancelledEvent(collection_bid) => {
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

        process_bid_placed_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            bid_placed_events,
        )
        .await?;

        process_bid_filled_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            bid_filled_events,
        )
        .await?;

        process_bid_cancelled_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            bid_cancelled_events,
        )
        .await?;

        process_ask_placed_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            ask_placed_events,
        )
        .await?;

        process_ask_filled_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            ask_filled_events,
        )
        .await?;

        process_ask_cancelled_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            ask_cancelled_events,
        )
        .await?;

        process_collection_bid_placed_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            collection_bid_placed_events,
        )
        .await?;

        process_collection_bid_filled_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            collection_bid_filled_events,
        )
        .await?;

        process_collection_bid_cancelled_events(
            self.pool.clone(),
            per_table_chunk_sizes.clone(),
            collection_bid_cancelled_events,
        )
        .await?;

        Ok(Some(transaction_context_data))
    }
}
