use aptos_indexer_processor_sdk::aptos_protos::transaction::v1::Event as EventPB;

use crate::{
    indexers::marketplace_indexer::extractor::ContractEvent,
    onchain_events::aptos_labs_contract_events::{
        collection_bid_event::{
            CollectionBidCancelledEventOnChain, CollectionBidFilledEventOnChain,
            CollectionBidPlacedEventOnChain,
        },
        nft_ask_event::{AskCancelledEventOnChain, AskFilledEventOnChain, AskPlacedEventOnChain},
        nft_bid_event::{BidCancelledEventOnChain, BidFilledEventOnChain, BidPlacedEventOnChain},
    },
};

pub fn parse_from_aptos_labs_contract_event(
    event_idx: i64,
    event: &EventPB,
    txn_version: i64,
    event_addr: String,
    event_type: String,
) -> Option<ContractEvent> {
    if event_type.starts_with(format!("{}::events::TokenOfferPlaced", event_addr).as_str()) {
        println!(
            "Aptos labs contract TokenOfferPlaced {}",
            event.data.as_str()
        );
        let parsed_event: BidPlacedEventOnChain = serde_json::from_str(event.data.as_str())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract TokenOfferPlaced, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidPlacedEvent(parsed_event.to_db_nft_bid(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type.starts_with(format!("{}::events::TokenOfferFilled", event_addr).as_str()) {
        println!(
            "Aptos labs contract TokenOfferFilled {}",
            event.data.as_str()
        );
        let parsed_event: BidFilledEventOnChain = serde_json::from_str(event.data.as_str())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract TokenOfferFilled, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidFilledEvent(parsed_event.to_db_nft_bid(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::events::TokenOfferCancelled", event_addr).as_str())
        || event_type.starts_with(format!("{}::events::TokenOfferCanceled", event_addr).as_str())
    {
        println!(
            "Aptos labs contract TokenOfferCancelled {}",
            event.data.as_str()
        );
        let parsed_event: BidCancelledEventOnChain = serde_json::from_str(event.data.as_str())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract TokenOfferCancelled, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidCancelledEvent(
            parsed_event.to_db_nft_bid(event_addr, txn_version, event_idx),
        ))
    } else if event_type.starts_with(format!("{}::events::ListingPlaced", event_addr).as_str()) {
        println!("Aptos labs contract ListingPlaced {}", event.data.as_str());
        let parsed_event: AskPlacedEventOnChain = serde_json::from_str(event.data.as_str())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract ListingPlaced, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskPlacedEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type.starts_with(format!("{}::events::ListingFilled", event_addr).as_str()) {
        println!("Aptos labs contract ListingFilled {}", event.data.as_str());
        let parsed_event: AskFilledEventOnChain = serde_json::from_str(event.data.as_str())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract ListingFilled, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskFilledEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type.starts_with(format!("{}::events::ListingCancelled", event_addr).as_str())
        || event_type.starts_with(format!("{}::events::ListingCanceled", event_addr).as_str())
    {
        println!(
            "Aptos labs contract ListingCancelled {}",
            event.data.as_str()
        );
        let parsed_event: AskCancelledEventOnChain = serde_json::from_str(event.data.as_str())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract ListingCancelled, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskCancelledEvent(
            parsed_event.to_db_nft_ask(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::events::CollectionOfferPlaced", event_addr).as_str())
    {
        println!(
            "Aptos labs contract CollectionOfferPlaced {}",
            event.data.as_str()
        );
        let parsed_event: CollectionBidPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Aptos labs contract Failed to parse CollectionOfferPlaced, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::CollectionBidPlacedEvent(
            parsed_event.to_db_collection_bid(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::events::CollectionOfferFilled", event_addr).as_str())
    {
        println!(
            "Aptos labs contract CollectionOfferFilled {}",
            event.data.as_str()
        );
        let parsed_event: CollectionBidFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract CollectionOfferFilled, {}",
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
    } else if event_type
        .starts_with(format!("{}::events::CollectionOfferCancelled", event_addr).as_str())
        || event_type
            .starts_with(format!("{}::events::CollectionOfferCanceled", event_addr).as_str())
    {
        println!(
            "Aptos labs contract CollectionOfferCancelled {}",
            event.data.as_str()
        );
        let parsed_event: CollectionBidCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Aptos labs contract CollectionOfferCancelled, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::CollectionBidCancelledEvent(
            parsed_event.to_db_collection_bid(event_addr, txn_version, event_idx),
        ))
    } else {
        None
    }
}
