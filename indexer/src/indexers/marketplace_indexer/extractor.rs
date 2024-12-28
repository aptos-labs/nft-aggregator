use ahash::AHashSet;
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::{
        transaction::TxnData, Event as EventPB, Transaction, WriteSetChange,
    },
    traits::{async_step::AsyncRunType, AsyncStep, NamedStep, Processable},
    types::transaction_context::TransactionContext,
    utils::{convert::standardize_address, errors::ProcessorError},
};
use async_trait::async_trait;
use rayon::prelude::*;

use crate::db_models::{
    collection_bids::{
        CollectionBid, CollectionBidOrderCancelledEventOnChain,
        CollectionBidOrderFilledEventOnChain, CollectionBidOrderPlacedEventOnChain,
        FilledCollectionBid,
    },
    nft_asks::{
        AskOrderCancelledEventOnChain, AskOrderFilledEventOnChain, AskOrderPlacedEventOnChain,
        NftAsk,
    },
    nft_bids::{
        BidOrderCancelledEventOnChain, BidOrderFilledEventOnChain, BidOrderPlacedEventOnChain,
        NftBid,
    },
};

/// Extractor is a step that extracts events and their metadata from transactions.
pub struct Extractor
where
    Self: Sized + Send + 'static,
{
    contract_addresses: AHashSet<String>,
}

impl Extractor {
    pub fn new(contract_addresses: Vec<String>) -> Self {
        Self {
            contract_addresses: contract_addresses.into_iter().collect(),
        }
    }
}

impl AsyncStep for Extractor {}

impl NamedStep for Extractor {
    fn name(&self) -> String {
        "Extractor".to_string()
    }
}

#[async_trait]
impl Processable for Extractor {
    type Input = Vec<Transaction>;
    type Output = TransactionContextData;
    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        item: TransactionContext<Vec<Transaction>>,
    ) -> Result<Option<TransactionContext<TransactionContextData>>, ProcessorError> {
        let results: Vec<(Vec<ContractEvent>, Vec<WriteSetChange>)> = item
            .data
            .par_iter()
            .map(|txn| {
                let tx_version = txn.version as i64;
                let txn_data = match txn.txn_data.as_ref() {
                    Some(data) => data,
                    None => {
                        tracing::warn!(
                            transaction_version = tx_version,
                            "Transaction data doesn't exist"
                        );
                        return (vec![], vec![]);
                    }
                };
                let raw_events = match txn_data {
                    TxnData::BlockMetadata(tx_inner) => &tx_inner.events,
                    TxnData::Genesis(tx_inner) => &tx_inner.events,
                    TxnData::User(tx_inner) => &tx_inner.events,
                    _ => &vec![],
                };

                let txn_events =
                    ContractEvent::from_events(&self.contract_addresses, raw_events, tx_version);

                (txn_events, vec![])
            })
            .collect::<Vec<(Vec<ContractEvent>, Vec<WriteSetChange>)>>();

        let (events, changes): (Vec<ContractEvent>, Vec<WriteSetChange>) =
            results.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut events_acc, mut changes_acc), (events, changes)| {
                    events_acc.extend(events);
                    changes_acc.extend(changes);
                    (events_acc, changes_acc)
                },
            );

        Ok(Some(TransactionContext {
            data: TransactionContextData { events, changes },
            metadata: item.metadata,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct TransactionContextData {
    pub events: Vec<ContractEvent>,
    pub changes: Vec<WriteSetChange>,
}

#[derive(Debug, Clone)]
pub enum ContractEvent {
    BidOrderPlacedEvent(NftBid),
    BidOrderFilledEvent(NftBid),
    BidOrderCancelledEvent(NftBid),
    AskOrderPlacedEvent(NftAsk),
    AskOrderFilledEvent(NftAsk),
    AskOrderCancelledEvent(NftAsk),
    CollectionBidOrderPlacedEvent(CollectionBid),
    CollectionBidOrderFilledEvent((CollectionBid, FilledCollectionBid)),
    CollectionBidOrderCancelledEvent(CollectionBid),
}

impl ContractEvent {
    fn from_event(
        contract_addresses: &AHashSet<String>,
        event_idx: i64,
        event: &EventPB,
        tx_version: i64,
    ) -> Option<Self> {
        // use standardize_address to pad the address in event type before processing
        let parts = event.type_str.split("::").collect::<Vec<_>>();
        let event_addr = standardize_address(parts[0]);
        let t = event_addr.clone() + "::" + parts[1] + "::" + parts[2];
        let should_include = contract_addresses.contains(event_addr.as_str());

        if should_include {
            if t.starts_with(format!("{}::events::BidOrderPlacedEvent", event_addr).as_str()) {
                println!("BidOrderPlacedEvent {}", event.data.as_str());
                let parsed_event: BidOrderPlacedEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse BidOrderPlacedEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::BidOrderPlacedEvent(
                    parsed_event.to_db_nft_bid(event_addr, tx_version, event_idx),
                ))
            } else if t.starts_with(format!("{}::events::BidOrderFilledEvent", event_addr).as_str())
            {
                println!("BidOrderFilledEvent {}", event.data.as_str());
                let parsed_event: BidOrderFilledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse BidOrderFilledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::BidOrderFilledEvent(
                    parsed_event.to_db_nft_bid(event_addr, tx_version, event_idx),
                ))
            } else if t
                .starts_with(format!("{}::events::BidOrderCancelledEvent", event_addr).as_str())
                || t.starts_with(format!("{}::events::BidOrderCanceledEvent", event_addr).as_str())
            {
                println!("BidOrderCancelledEvent {}", event.data.as_str());
                let parsed_event: BidOrderCancelledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse BidOrderCancelledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::BidOrderCancelledEvent(
                    parsed_event.to_db_nft_bid(event_addr, tx_version, event_idx),
                ))
            } else if t.starts_with(format!("{}::events::AskOrderPlacedEvent", event_addr).as_str())
            {
                println!("AskOrderPlacedEvent {}", event.data.as_str());
                let parsed_event: AskOrderPlacedEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse AskOrderPlacedEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::AskOrderPlacedEvent(
                    parsed_event.to_db_nft_ask(event_addr, tx_version, event_idx),
                ))
            } else if t.starts_with(format!("{}::events::AskOrderFilledEvent", event_addr).as_str())
            {
                println!("AskOrderFilledEvent {}", event.data.as_str());
                let parsed_event: AskOrderFilledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse AskOrderFilledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::AskOrderFilledEvent(
                    parsed_event.to_db_nft_ask(event_addr, tx_version, event_idx),
                ))
            } else if t
                .starts_with(format!("{}::events::AskOrderCancelledEvent", event_addr).as_str())
                || t.starts_with(format!("{}::events::AskOrderCanceledEvent", event_addr).as_str())
            {
                println!("AskOrderCancelledEvent {}", event.data.as_str());
                let parsed_event: AskOrderCancelledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse AskOrderCancelledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::AskOrderCancelledEvent(
                    parsed_event.to_db_nft_ask(event_addr, tx_version, event_idx),
                ))
            } else if t.starts_with(
                format!("{}::events::CollectionBidOrderPlacedEvent", event_addr).as_str(),
            ) {
                println!("CollectionBidOrderPlacedEvent {}", event.data.as_str());
                let parsed_event: CollectionBidOrderPlacedEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse CollectionBidOrderPlacedEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::CollectionBidOrderPlacedEvent(
                    parsed_event.to_db_collection_bid(event_addr, tx_version, event_idx),
                ))
            } else if t.starts_with(
                format!("{}::events::CollectionBidOrderFilledEvent", event_addr).as_str(),
            ) {
                println!("CollectionBidOrderFilledEvent {}", event.data.as_str());
                let parsed_event: CollectionBidOrderFilledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse CollectionBidOrderFilledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::CollectionBidOrderFilledEvent(
                    parsed_event.to_db_collection_bid_and_filled_collection_bid(
                        event_addr, tx_version, event_idx,
                    ),
                ))
            } else if t.starts_with(
                format!("{}::events::CollectionBidOrderCancelledEvent", event_addr).as_str(),
            ) || t.starts_with(
                format!("{}::events::CollectionBidOrderCanceledEvent", event_addr).as_str(),
            ) {
                println!("CollectionBidOrderCancelledEvent {}", event.data.as_str());
                let parsed_event: CollectionBidOrderCancelledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse CollectionBidOrderCancelledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::CollectionBidOrderCancelledEvent(
                    parsed_event.to_db_collection_bid(event_addr, tx_version, event_idx),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn from_events(
        contract_addresses: &AHashSet<String>,
        events: &[EventPB],
        tx_version: i64,
    ) -> Vec<Self> {
        events
            .iter()
            .enumerate()
            .filter_map(|(idx, event)| {
                Self::from_event(contract_addresses, idx as i64, event, tx_version)
            })
            .collect()
    }
}
