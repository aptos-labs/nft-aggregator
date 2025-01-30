use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::{
    db_models::{activities::Activity, nft_bids::NftBid},
    onchain_events::aptos_labs_contract_events::shared::MoveObject,
    utils::aptos_utils::{ActivityType, NFTStandard, OrderStatus, PaymentTokenType, APT_COIN},
};

// Tradeport v2 InsertTokenBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV2BidPlacedEventOnChain {
    pub timestamp: String,
    pub bid: MoveObject,
    pub bid_buyer: String,
    pub token: MoveObject,
    pub price: String,
}

// Tradeport v2 AcceptTokenBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV2BidFilledEventOnChain {
    pub timestamp: String,
    pub bid: MoveObject,
    pub bid_buyer: String,
    pub bid_seller: String,
    pub token: MoveObject,
    pub price: String,
}

// Tradeport v2 DeleteTokenBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV2BidCancelledEventOnChain {
    pub timestamp: String,
    pub bid: MoveObject,
    pub bid_buyer: String,
    pub token: MoveObject,
    pub price: String,
}

impl TradeportV2BidPlacedEventOnChain {
    pub fn to_db_nft_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (NftBid, Activity) {
        (
            NftBid {
                bid_obj_addr: standardize_address(self.bid.inner.as_str()),
                nft_id: self.token.inner.clone(),
                nft_name: "".to_string(),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr: marketplace_addr.clone(),
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                seller_addr: "".to_string(),
                price: self.price.parse().unwrap(),
                commission: 0,
                royalties: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: self.timestamp.parse().unwrap(),
                order_placed_tx_version: tx_version,
                order_placed_event_idx: event_idx,
                order_filled_timestamp: 0,
                order_filled_tx_version: 0,
                order_filled_event_idx: 0,
                order_cancelled_timestamp: 0,
                order_cancelled_tx_version: 0,
                order_cancelled_event_idx: 0,
                order_status: OrderStatus::Open as i32,
                order_expiration_timestamp: 0,
            },
            Activity {
                nft_id: standardize_address(self.token.inner.as_str()),
                nft_name: "".to_string(),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr,
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                seller_addr: "".to_string(),
                price: self.price.parse().unwrap(),
                royalties: 0,
                commission: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                activity_timestamp: self.timestamp.parse().unwrap(),
                activity_tx_version: tx_version,
                activity_event_idx: event_idx,
                activity_type: ActivityType::NFTAskPlaced as i32,
            },
        )
    }
}

impl TradeportV2BidFilledEventOnChain {
    pub fn to_db_nft_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (NftBid, Activity) {
        (
            NftBid {
                bid_obj_addr: standardize_address(self.bid.inner.as_str()),
                nft_id: standardize_address(self.token.inner.as_str()),
                nft_name: "".to_string(),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr: marketplace_addr.clone(),
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                seller_addr: standardize_address(self.bid_seller.as_str()),
                price: self.price.parse().unwrap(),
                commission: 0,
                royalties: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: 0,
                order_placed_tx_version: 0,
                order_placed_event_idx: 0,
                order_filled_timestamp: self.timestamp.parse().unwrap(),
                order_filled_tx_version: tx_version,
                order_filled_event_idx: event_idx,
                order_cancelled_timestamp: 0,
                order_cancelled_tx_version: 0,
                order_cancelled_event_idx: 0,
                order_status: OrderStatus::Filled as i32,
                order_expiration_timestamp: 0,
            },
            Activity {
                nft_id: standardize_address(self.token.inner.as_str()),
                nft_name: "".to_string(),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr,
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                seller_addr: standardize_address(self.bid_seller.as_str()),
                price: self.price.parse().unwrap(),
                royalties: 0,
                commission: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                activity_timestamp: self.timestamp.parse().unwrap(),
                activity_tx_version: tx_version,
                activity_event_idx: event_idx,
                activity_type: ActivityType::NFTBidFilled as i32,
            },
        )
    }
}

impl TradeportV2BidCancelledEventOnChain {
    pub fn to_db_nft_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (NftBid, Activity) {
        (
            NftBid {
                bid_obj_addr: standardize_address(self.bid.inner.as_str()),
                nft_id: standardize_address(self.token.inner.as_str()),
                nft_name: "".to_string(),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr: marketplace_addr.clone(),
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                seller_addr: "".to_string(),
                price: self.price.parse().unwrap(),
                commission: 0,
                royalties: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: 0,
                order_placed_tx_version: 0,
                order_placed_event_idx: 0,
                order_filled_timestamp: 0,
                order_filled_tx_version: 0,
                order_filled_event_idx: 0,
                order_cancelled_timestamp: self.timestamp.parse().unwrap(),
                order_cancelled_tx_version: tx_version,
                order_cancelled_event_idx: event_idx,
                order_status: OrderStatus::Cancelled as i32,
                order_expiration_timestamp: 0,
            },
            Activity {
                nft_id: standardize_address(self.token.inner.as_str()),
                nft_name: "".to_string(),
                collection_addr: "".to_string(),
                collection_creator_addr: "".to_string(),
                collection_name: "".to_string(),
                nft_standard: NFTStandard::V2 as i32,
                marketplace_addr,
                buyer_addr: standardize_address(self.bid_buyer.as_str()),
                seller_addr: "".to_string(),
                price: self.price.parse().unwrap(),
                royalties: 0,
                commission: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                activity_timestamp: self.timestamp.parse().unwrap(),
                activity_tx_version: tx_version,
                activity_event_idx: event_idx,
                activity_type: ActivityType::NFTBidCancelled as i32,
            },
        )
    }
}
