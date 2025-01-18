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
    collection_bids::{CollectionBid, FilledCollectionBid},
    nft_asks::NftAsk,
    nft_bids::NftBid,
};

use super::event_parsers::{
    aptos_labs_contract_event_parser::parse_from_aptos_labs_contract_event,
    tradeport_contract_v1_event_parser::parse_from_tradeport_v1_contract_event,
    tradeport_contract_v2_event_parser::parse_from_tradeport_v2_contract_event,
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
            let event_type = event_addr.clone() + "::" + parts[1] + "::" + parts[2];
            parse_from_aptos_labs_contract_event(
                event_idx,
                event,
                txn_version,
                event_addr.clone(),
                event_type.clone(),
            )
            .or_else(|| {
                parse_from_tradeport_v1_contract_event(
                    event_idx,
                    event,
                    txn_version,
                    event_addr.clone(),
                    event_type.clone(),
                )
            })
            .or_else(|| {
                parse_from_tradeport_v2_contract_event(
                    event_idx,
                    event,
                    txn_version,
                    event_addr,
                    event_type,
                )
            })
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
