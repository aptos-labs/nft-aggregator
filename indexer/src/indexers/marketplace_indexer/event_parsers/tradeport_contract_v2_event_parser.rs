use aptos_indexer_processor_sdk::aptos_protos::transaction::v1::Event as EventPB;

use crate::{
    indexers::marketplace_indexer::extractor::ContractEvent,
    onchain_events::tradeport_contract_events::{
        collection_bid_event_v2::{
            TradeportV2CollectionBidCancelledEventOnChain,
            TradeportV2CollectionBidFilledEventOnChain, TradeportV2CollectionBidPlacedEventOnChain,
        },
        nft_ask_event_v2::{
            TradeportV2AskCancelledEventOnChain, TradeportV2AskFilledEventOnChain,
            TradeportV2AskPlacedEventOnChain,
        },
        nft_bid_event_v2::{
            TradeportV2BidCancelledEventOnChain, TradeportV2BidFilledEventOnChain,
            TradeportV2BidPlacedEventOnChain,
        },
    },
};

pub fn parse_from_tradeport_v2_contract_event(
    event_idx: i64,
    event: &EventPB,
    txn_version: i64,
    event_addr: String,
    event_type: String,
) -> Option<ContractEvent> {
    if event_type.starts_with(format!("{}::biddings_v2::InsertTokenBidEvent", event_addr).as_str())
    {
        println!("Tradeport v2 InsertTokenBidEvent {}", event.data.as_str());
        let parsed_event: TradeportV2BidPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 InsertTokenBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidPlacedEvent(parsed_event.to_db_nft_bid(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::biddings_v2::AcceptTokenBidEvent", event_addr).as_str())
    {
        println!("Tradeport v2 AcceptTokenBidEvent {}", event.data.as_str());
        let parsed_event: TradeportV2BidFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 AcceptTokenBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidFilledEvent(parsed_event.to_db_nft_bid(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::biddings_v2::DeleteTokenBidEvent", event_addr).as_str())
    {
        println!("Tradeport v2 DeleteTokenBidEvent {}", event.data.as_str());
        let parsed_event: TradeportV2BidCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 DeleteTokenBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::BidCancelledEvent(
            parsed_event.to_db_nft_bid(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::listings_v2::InsertListingEvent", event_addr).as_str())
    {
        println!("Tradeport v2 InsertListingEvent {}", event.data.as_str());
        let parsed_event: TradeportV2AskPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 InsertListingEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskPlacedEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type.starts_with(format!("{}::listings_v2::BuyEvent", event_addr).as_str()) {
        println!("Tradeport v2 BuyEvent {}", event.data.as_str());
        let parsed_event: TradeportV2AskFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 BuyEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskFilledEvent(parsed_event.to_db_nft_ask(
            event_addr,
            txn_version,
            event_idx,
        )))
    } else if event_type
        .starts_with(format!("{}::listings_v2::DeleteListingEvent", event_addr).as_str())
    {
        println!("Tradeport v2 DeleteListingEvent {}", event.data.as_str());
        let parsed_event: TradeportV2AskCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 DeleteListingEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::AskCancelledEvent(
            parsed_event.to_db_nft_ask(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::biddings_v2::InsertCollectionBidEvent", event_addr).as_str())
    {
        println!(
            "Tradeport v2 InsertCollectionBidEvent {}",
            event.data.as_str()
        );
        let parsed_event: TradeportV2CollectionBidPlacedEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 InsertCollectionBidEvent, {}",
                    event.data.as_str()
                )
            });
        Some(ContractEvent::CollectionBidPlacedEvent(
            parsed_event.to_db_collection_bid(event_addr, txn_version, event_idx),
        ))
    } else if event_type
        .starts_with(format!("{}::biddings_v2::AcceptCollectionBidEvent", event_addr).as_str())
    {
        println!(
            "Tradeport v2 AcceptCollectionBidEvent {}",
            event.data.as_str()
        );
        let parsed_event: TradeportV2CollectionBidFilledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 AcceptCollectionBidEvent, {}",
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
        .starts_with(format!("{}::biddings_v2::DeleteCollectionBidEvent", event_addr).as_str())
    {
        println!(
            "Tradeport v2 DeleteCollectionBidEvent {}",
            event.data.as_str()
        );
        let parsed_event: TradeportV2CollectionBidCancelledEventOnChain =
            serde_json::from_str(event.data.as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to parse Tradeport v2 DeleteCollectionBidEvent, {}",
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
