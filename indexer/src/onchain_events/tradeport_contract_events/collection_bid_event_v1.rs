use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::{
    db_models::{collection_bids::CollectionBid, filled_collection_bids::FilledCollectionBid},
    utils::aptos_utils::{NFTStandard, OrderStatus, PaymentTokenType, APT_COIN},
};

use super::shared::{generate_collection_bid_order_id_for_nft_v1, NftV1CollectionId, NftV1TokenId};

// Tradeport v1 InsertCollectionBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1CollectionBidPlacedEventOnChain {
    pub timestamp: String,
    pub nonce: String,
    pub bid_buyer: String,
    pub collection_id: NftV1CollectionId,
    pub amount: String,
    pub price: String,
}

// Tradeport v1 AcceptCollectionBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1CollectionBidFilledEventOnChain {
    pub timestamp: String,
    pub nonce: String,
    pub bid_buyer: String,
    pub bid_seller: String,
    pub token_id: NftV1TokenId,
    pub price: String,
}

// Tradeport v1 DeleteCollectionBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1CollectionBidCancelledEventOnChain {
    pub timestamp: String,
    pub nonce: String,
    pub bid_buyer: String,
    pub collection_id: NftV1CollectionId,
    pub amount: String,
    pub price: String,
}

impl TradeportV1CollectionBidPlacedEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> CollectionBid {
        CollectionBid {
            bid_obj_addr: generate_collection_bid_order_id_for_nft_v1(self.nonce.clone()),
            collection_addr: "".to_string(),
            collection_creator_addr: standardize_address(
                self.collection_id.collection_creator.as_str(),
            ),
            collection_name: self.collection_id.collection_name.clone(),
            nft_standard: NFTStandard::V1 as i32,
            marketplace_addr,
            total_nft_amount: self.amount.parse().unwrap(),
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

impl TradeportV1CollectionBidFilledEventOnChain {
    pub fn to_db_collection_bid_and_filled_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (CollectionBid, FilledCollectionBid) {
        (
            CollectionBid {
                bid_obj_addr: generate_collection_bid_order_id_for_nft_v1(self.nonce.clone()),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: 1,
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
                bid_obj_addr: generate_collection_bid_order_id_for_nft_v1(self.nonce.clone()),
                nft_id: self.token_id.property_version.clone(),
                nft_name: self.token_id.token_data_id.name.clone(),
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

impl TradeportV1CollectionBidCancelledEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> CollectionBid {
        CollectionBid {
            bid_obj_addr: generate_collection_bid_order_id_for_nft_v1(self.nonce.clone()),
            collection_addr: "".to_string(),
            collection_creator_addr: "".to_string(),
            collection_name: "".to_string(),
            nft_standard: 1,
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
