use aptos_indexer_processor_sdk::aptos_protos::transaction::v1::Event as EventPB;

use crate::{
    indexers::marketplace_indexer::extractor::ContractEvent,
    onchain_events::tradeport_contract_events::{
        collection_bid_event_v1::{
            TradeportV1CollectionBidCancelledEventOnChain,
            TradeportV1CollectionBidFilledEventOnChain, TradeportV1CollectionBidPlacedEventOnChain,
        },
        nft_ask_event_v1::{
            TradeportV1AskCancelledEventOnChain, TradeportV1AskFilledEventOnChain,
            TradeportV1AskPlacedEventOnChain, TradeportV1AskUpdatedEventOnChain,
        },
        nft_bid_event_v1::{
            TradeportV1BidCancelledEventOnChain, TradeportV1BidFilledEventOnChain,
            TradeportV1BidPlacedEventOnChain,
        },
    },
};

pub fn parse_from_tradeport_v1_contract_event(
    event_idx: i64,
    event: &EventPB,
    txn_version: i64,
    event_addr: String,
    event_type: String,
) -> Option<ContractEvent> {
    if event_type.starts_with(format!("{}::biddings::InsertTokenBidEvent", event_addr).as_str()) {
        println!("Tradeport v1 InsertTokenBidEvent {}", event.data.as_str());
        let parsed_event: TradeportV1BidPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 InsertTokenBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidPlacedEvent(parsed_event.to_db_nft_bid(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::biddings::AcceptTokenBidEvent", event_addr).as_str())
    {
        println!("Tradeport v1 AcceptTokenBidEvent {}", event.data.as_str());
        let parsed_event: TradeportV1BidFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 AcceptTokenBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidFilledEvent(parsed_event.to_db_nft_bid(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::biddings::DeleteTokenBidEvent", event_addr).as_str())
    {
        println!("Tradeport v1 DeleteTokenBidEvent {}", event.data.as_str());
        let parsed_event: TradeportV1BidCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 DeleteTokenBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidCancelledEvent(
            parsed_event.to_db_nft_bid(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::listings::InsertListingEvent", event_addr).as_str())
    {
        println!("Tradeport v1 InsertListingEvent {}", event.data.as_str());
        let parsed_event: TradeportV1AskPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 InsertListingEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskPlacedEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::listings::UpdateListingEvent", event_addr).as_str())
    {
        println!("Tradeport v1 UpdateListingEvent {}", event.data.as_str());
        let parsed_event: TradeportV1AskUpdatedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 UpdateListingEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskPlacedEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type.starts_with(format!("{}::listings::BuyEvent", event_addr).as_str()) {
        println!("Tradeport v1 BuyEvent {}", event.data.as_str());
        let parsed_event: TradeportV1AskFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 BuyEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskFilledEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::listings::DeleteListingEvent", event_addr).as_str())
    {
        println!("Tradeport v1 DeleteListingEvent {}", event.data.as_str());
        let parsed_event: TradeportV1AskCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 DeleteListingEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskCancelledEvent(
            parsed_event.to_db_nft_ask(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::biddings::InsertCollectionBidEvent", event_addr).as_str())
    {
        println!(
            "Tradeport v1 InsertCollectionBidEvent {}",
            event.data.as_str()
        );
        let parsed_event: TradeportV1CollectionBidPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 InsertCollectionBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::CollectionBidPlacedEvent(
            parsed_event.to_db_collection_bid(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::biddings::AcceptCollectionBidEvent", event_addr).as_str())
    {
        println!(
            "Tradeport v1 AcceptCollectionBidEvent {}",
            event.data.as_str()
        );
        let parsed_event: TradeportV1CollectionBidFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 AcceptCollectionBidEvent, {}",
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
        .starts_with(format!("{}::biddings::DeleteCollectionBidEvent", event_addr).as_str())
    {
        println!(
            "Tradeport v1 DeleteCollectionBidEvent {}",
            event.data.as_str()
        );
        let parsed_event: TradeportV1CollectionBidCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v1 DeleteCollectionBidEvent, {}",
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
