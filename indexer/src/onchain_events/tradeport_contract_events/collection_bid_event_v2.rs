use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::{
    db_models::collection_bids::{CollectionBid, FilledCollectionBid},
    onchain_events::aptos_labs_contract_events::shared::MoveObject,
    utils::aptos_utils::{NFTStandard, OrderStatus, PaymentTokenType, APT_COIN},
};

// Tradeport v2 InsertCollectionBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV2CollectionBidPlacedEventOnChain {
    pub timestamp: String,
    pub bid: MoveObject,
    pub bid_buyer: String,
    pub collection: MoveObject,
    pub price: String,
}

// Tradeport v2 AcceptCollectionBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV2CollectionBidFilledEventOnChain {
    pub timestamp: String,
    pub bid: MoveObject,
    pub bid_buyer: String,
    pub bid_seller: String,
    pub token: MoveObject,
    pub price: String,
}

// Tradeport v2 DeleteCollectionBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV2CollectionBidCancelledEventOnChain {
    pub timestamp: String,
    pub bid: MoveObject,
    pub bid_buyer: String,
    pub collection: MoveObject,
    pub price: String,
}

impl TradeportV2CollectionBidPlacedEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> CollectionBid {
        CollectionBid {
            bid_obj_addr: standardize_address(self.bid.inner.as_str()),
            collection_addr: standardize_address(self.collection.inner.as_str()),
            collection_creator_addr: "".to_string(),
            collection_name: "".to_string(),
            nft_standard: NFTStandard::V2 as i32,
            marketplace_addr,
            total_nft_amount: 1,
            buyer_addr: standardize_address(self.bid_buyer.as_str()),
            price: self.price.parse().unwrap(),
            payment_token: APT_COIN.to_string(),
            payment_token_type: PaymentTokenType::Coin as i32,
            order_placed_timestamp: self.timestamp.parse().unwrap(),
            order_placed_tx_version: tx_version,
            order_placed_event_idx: event_idx,
            latest_order_filled_timestamp: 0,
            latest_order_filled_tx_version: 0,
            latest_order_filled_event_idx: 0,
            order_cancelled_timestamp: 0,
            order_cancelled_tx_version: 0,
            order_cancelled_event_idx: 0,
            order_status: OrderStatus::Open as i32,
            order_expiration_timestamp: 0,
        }
    }
}

impl TradeportV2CollectionBidFilledEventOnChain {
    pub fn to_db_collection_bid_and_filled_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (CollectionBid, FilledCollectionBid) {
        (
            CollectionBid {
                bid_obj_addr: standardize_address(self.bid.inner.as_str()),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr,
                total_nft_amount: 0,
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                price: self.price.parse().unwrap(),
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: 0,
                order_placed_tx_version: 0,
                order_placed_event_idx: 0,
                latest_order_filled_timestamp: self.timestamp.parse().unwrap(),
                latest_order_filled_tx_version: tx_version,
                latest_order_filled_event_idx: event_idx,
                order_cancelled_timestamp: 0,
                order_cancelled_tx_version: 0,
                order_cancelled_event_idx: 0,
                order_status: OrderStatus::Open as i32,
                order_expiration_timestamp: 0,
            },
            FilledCollectionBid {
                bid_obj_addr: standardize_address(self.bid.inner.as_str()),
                nft_id: self.token.inner.clone(),
                nft_name: "".to_string(),
                seller_addr: standardize_address(self.bid_seller.as_str()),
                price: self.price.parse().unwrap(),
                royalties: 0,
                commission: 0,
                order_filled_timestamp: self.timestamp.parse().unwrap(),
                order_filled_tx_version: tx_version,
                order_filled_event_idx: event_idx,
            },
        )
    }
}

impl TradeportV2CollectionBidCancelledEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> CollectionBid {
        CollectionBid {
            bid_obj_addr: standardize_address(self.collection.inner.as_str()),
            collection_addr: "".to_string(),
            collection_creator_addr: "".to_string(),
            collection_name: "".to_string(),
            nft_standard: NFTStandard::V2 as i32,
            marketplace_addr,
            total_nft_amount: 0,
            buyer_addr: standardize_address(self.bid_buyer.as_str()),
            price: self.price.parse().unwrap(),
            payment_token: APT_COIN.to_string(),
            payment_token_type: PaymentTokenType::Coin as i32,
            order_placed_timestamp: 0,
            order_placed_tx_version: 0,
            order_placed_event_idx: 0,
            latest_order_filled_timestamp: 0,
            latest_order_filled_tx_version: 0,
            latest_order_filled_event_idx: 0,
            order_cancelled_timestamp: self.timestamp.parse().unwrap(),
            order_cancelled_tx_version: tx_version,
            order_cancelled_event_idx: event_idx,
            order_status: OrderStatus::Cancelled as i32,
            order_expiration_timestamp: 0,
        }
    }
}
