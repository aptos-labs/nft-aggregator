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
        CollectionBid, CollectionBidCancelledEventOnChain, CollectionBidFilledEventOnChain,
        CollectionBidPlacedEventOnChain, FilledCollectionBid,
    },
    nft_asks::{AskCancelledEventOnChain, AskFilledEventOnChain, AskPlacedEventOnChain, NftAsk},
    nft_bids::{BidCancelledEventOnChain, BidFilledEventOnChain, BidPlacedEventOnChain, NftBid},
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
                let txn_version = txn.version as i64;
                match txn.info.as_ref() {
                    Some(info) => {
                        if !info.success {
                            return (vec![], vec![]);
                        }
                    }
                    None => {
                        tracing::warn!(
                            transaction_version = txn_version,
                            "Transaction info doesn't exist"
                        );
                        return (vec![], vec![]);
                    }
                };
                let txn_data = match txn.txn_data.as_ref() {
                    Some(data) => data,
                    None => {
                        tracing::warn!(
                            transaction_version = txn_version,
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
                    ContractEvent::from_events(&self.contract_addresses, raw_events, txn_version);

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
    BidPlacedEvent(NftBid),
    BidFilledEvent(NftBid),
    BidCancelledEvent(NftBid),
    AskPlacedEvent(NftAsk),
    AskFilledEvent(NftAsk),
    AskCancelledEvent(NftAsk),
    CollectionBidPlacedEvent(CollectionBid),
    CollectionBidFilledEvent((CollectionBid, FilledCollectionBid)),
    CollectionBidCancelledEvent(CollectionBid),
}

impl ContractEvent {
    fn from_event(
        contract_addresses: &AHashSet<String>,
        event_idx: i64,
        event: &EventPB,
        txn_version: i64,
    ) -> Option<Self> {
        // use standardize_address to pad the address in event type before processing
        let parts = event.type_str.split("::").collect::<Vec<_>>();
        let event_addr = standardize_address(parts[0]);

        if contract_addresses.contains(event_addr.as_str()) {
            let t = event_addr.clone() + "::" + parts[1] + "::" + parts[2];
            if t.starts_with(format!("{}::events::TokenOfferPlaced", event_addr).as_str()) {
                println!("BidPlacedEvent {}", event.data.as_str());
                let parsed_event: BidPlacedEventOnChain = serde_json::from_str(event.data.as_str())
                    .unwrap_or_else(|_| {
                        panic!("Failed to parse BidPlacedEvent, {}", event.data.as_str())
                    });
                Some(ContractEvent::BidPlacedEvent(parsed_event.to_db_nft_bid(
                    event_addr,
                    txn_version,
                    event_idx,
                )))
            } else if t.starts_with(format!("{}::events::TokenOfferFilled", event_addr).as_str()) {
                println!("BidFilledEvent {}", event.data.as_str());
                let parsed_event: BidFilledEventOnChain = serde_json::from_str(event.data.as_str())
                    .unwrap_or_else(|_| {
                        panic!("Failed to parse BidFilledEvent, {}", event.data.as_str())
                    });
                Some(ContractEvent::BidFilledEvent(parsed_event.to_db_nft_bid(
                    event_addr,
                    txn_version,
                    event_idx,
                )))
            } else if t.starts_with(format!("{}::events::TokenOfferCanceled", event_addr).as_str())
                || t.starts_with(format!("{}::events::TokenOfferCancelled", event_addr).as_str())
            {
                println!("BidCancelledEvent {}", event.data.as_str());
                let parsed_event: BidCancelledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!("Failed to parse BidCancelledEvent, {}", event.data.as_str())
                    });
                Some(ContractEvent::BidCancelledEvent(
                    parsed_event.to_db_nft_bid(event_addr, txn_version, event_idx),
                ))
            } else if t.starts_with(format!("{}::events::ListingPlaced", event_addr).as_str()) {
                println!("AskPlacedEvent {}", event.data.as_str());
                let parsed_event: AskPlacedEventOnChain = serde_json::from_str(event.data.as_str())
                    .unwrap_or_else(|_| {
                        panic!("Failed to parse AskPlacedEvent, {}", event.data.as_str())
                    });
                Some(ContractEvent::AskPlacedEvent(parsed_event.to_db_nft_ask(
                    event_addr,
                    txn_version,
                    event_idx,
                )))
            } else if t.starts_with(format!("{}::events::ListingFilled", event_addr).as_str()) {
                println!("AskFilledEvent {}", event.data.as_str());
                let parsed_event: AskFilledEventOnChain = serde_json::from_str(event.data.as_str())
                    .unwrap_or_else(|_| {
                        panic!("Failed to parse AskFilledEvent, {}", event.data.as_str())
                    });
                Some(ContractEvent::AskFilledEvent(parsed_event.to_db_nft_ask(
                    event_addr,
                    txn_version,
                    event_idx,
                )))
            } else if t.starts_with(format!("{}::events::ListingCancelled", event_addr).as_str())
                || t.starts_with(format!("{}::events::ListingCanceled", event_addr).as_str())
            {
                println!("AskCancelledEvent {}", event.data.as_str());
                let parsed_event: AskCancelledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!("Failed to parse AskCancelledEvent, {}", event.data.as_str())
                    });
                Some(ContractEvent::AskCancelledEvent(
                    parsed_event.to_db_nft_ask(event_addr, txn_version, event_idx),
                ))
            } else if t
                .starts_with(format!("{}::events::CollectionOfferPlaced", event_addr).as_str())
            {
                println!("CollectionBidPlacedEvent {}", event.data.as_str());
                let parsed_event: CollectionBidPlacedEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse CollectionBidPlacedEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::CollectionBidPlacedEvent(
                    parsed_event.to_db_collection_bid(event_addr, txn_version, event_idx),
                ))
            } else if t
                .starts_with(format!("{}::events::CollectionOfferFilled", event_addr).as_str())
            {
                println!("CollectionBidFilledEvent {}", event.data.as_str());
                let parsed_event: CollectionBidFilledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse CollectionBidFilledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::CollectionBidFilledEvent(
                    parsed_event.to_db_collection_bid_and_filled_collection_bid(
                        event_addr,
                        txn_version,
                        event_idx,
                    ),
                ))
            } else if t
                .starts_with(format!("{}::events::CollectionOfferCancelled", event_addr).as_str())
                || t.starts_with(
                    format!("{}::events::CollectionOfferCanceled", event_addr).as_str(),
                )
            {
                println!("CollectionBidCancelledEvent {}", event.data.as_str());
                let parsed_event: CollectionBidCancelledEventOnChain =
                    serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Failed to parse CollectionBidCancelledEvent, {}",
                            event.data.as_str()
                        )
                    });
                Some(ContractEvent::CollectionBidCancelledEvent(
                    parsed_event.to_db_collection_bid(event_addr, txn_version, event_idx),
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
        txn_version: i64,
    ) -> Vec<Self> {
        events
            .iter()
            .enumerate()
            .filter_map(|(idx, event)| {
                Self::from_event(contract_addresses, idx as i64, event, txn_version)
            })
            .collect()
    }
}
