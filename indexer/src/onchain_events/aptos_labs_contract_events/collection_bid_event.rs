use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::{
    db_models::{
        activities::Activity, collection_bids::CollectionBid,
        filled_collection_bids::FilledCollectionBid,
    },
    utils::{
        aptos_utils::{ActivityType, OrderStatus, PaymentTokenType, APT_COIN},
        time_utils::get_unix_timestamp_in_secs,
    },
};

use super::shared::{CollectionMetadataOnChain, TokenMetadataOnChain};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionBidPlacedEventOnChain {
    pub collection_offer: String,
    pub purchaser: String,
    pub price: String,
    pub token_amount: String,
    pub collection_metadata: CollectionMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionBidFilledEventOnChain {
    pub collection_offer: String,
    pub purchaser: String,
    pub seller: String,
    pub price: String,
    pub royalties: String,
    pub commission: String,
    pub token_metadata: TokenMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionBidCancelledEventOnChain {
    pub collection_offer: String,
    pub purchaser: String,
    pub price: String,
    pub remaining_token_amount: String,
    pub collection_metadata: CollectionMetadataOnChain,
}

impl CollectionBidPlacedEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (CollectionBid, Activity) {
        let time_now = get_unix_timestamp_in_secs();
        (
            CollectionBid {
                bid_obj_addr: standardize_address(self.collection_offer.as_str()),
                collection_addr: self.collection_metadata.get_collection_addr().clone(),
                collection_creator_addr: standardize_address(
                    self.collection_metadata.creator_address.as_str(),
                ),
                collection_name: self.collection_metadata.collection_name.clone(),
                nft_standard: self.collection_metadata.get_nft_standard(),
                marketplace_addr: marketplace_addr.clone(),
                total_nft_amount: self.token_amount.parse().unwrap(),
                buyer_addr: standardize_address(self.purchaser.as_str()),
                price: self.price.parse().unwrap(),
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: time_now,
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
            },
            Activity {
                nft_id: "".to_string(),
                nft_name: "".to_string(),
                collection_addr: self.collection_metadata.get_collection_addr().clone(),
                collection_creator_addr: standardize_address(
                    self.collection_metadata.creator_address.as_str(),
                ),
                collection_name: self.collection_metadata.collection_name.clone(),
                nft_standard: self.collection_metadata.get_nft_standard(),
                marketplace_addr,
                buyer_addr: standardize_address(self.purchaser.as_str()),
                seller_addr: "".to_string(),
                price: self.price.parse().unwrap(),
                royalties: 0,
                commission: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                activity_timestamp: time_now,
                activity_tx_version: tx_version,
                activity_event_idx: event_idx,
                activity_type: ActivityType::CollectionBidPlaced as i32,
            },
        )
    }
}

impl CollectionBidFilledEventOnChain {
    pub fn to_db_collection_bid_and_filled_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (CollectionBid, FilledCollectionBid, Activity) {
        let time_now = get_unix_timestamp_in_secs();
        (
            CollectionBid {
                bid_obj_addr: standardize_address(self.collection_offer.as_str()),
                collection_addr: self.token_metadata.get_collection_addr().clone(),
                collection_creator_addr: standardize_address(
                    self.token_metadata.creator_address.as_str(),
                ),
                collection_name: self.token_metadata.collection_name.clone(),
                nft_standard: self.token_metadata.get_nft_standard(),
                marketplace_addr: marketplace_addr.clone(),
                total_nft_amount: 0,
                buyer_addr: standardize_address(self.purchaser.as_str()),
                price: self.price.parse().unwrap(),
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: 0,
                order_placed_tx_version: 0,
                order_placed_event_idx: 0,
                latest_order_filled_timestamp: time_now,
                latest_order_filled_tx_version: tx_version,
                latest_order_filled_event_idx: event_idx,
                order_cancelled_timestamp: 0,
                order_cancelled_tx_version: 0,
                order_cancelled_event_idx: 0,
                order_status: OrderStatus::Open as i32,
                order_expiration_timestamp: 0,
            },
            FilledCollectionBid {
                bid_obj_addr: standardize_address(self.collection_offer.as_str()),
                nft_id: self.token_metadata.get_id(),
                nft_name: self.token_metadata.token_name.clone(),
                seller_addr: standardize_address(self.seller.as_str()),
                price: self.price.parse().unwrap(),
                royalties: self.royalties.parse().unwrap(),
                commission: self.commission.parse().unwrap(),
                order_filled_timestamp: time_now,
                order_filled_tx_version: tx_version,
                order_filled_event_idx: event_idx,
            },
            Activity {
                nft_id: self.token_metadata.get_id(),
                nft_name: self.token_metadata.token_name.clone(),
                collection_addr: self.token_metadata.get_collection_addr().clone(),
                collection_creator_addr: standardize_address(
                    self.token_metadata.creator_address.as_str(),
                ),
                collection_name: self.token_metadata.collection_name.clone(),
                nft_standard: self.token_metadata.get_nft_standard(),
                marketplace_addr,
                buyer_addr: standardize_address(self.purchaser.as_str()),
                seller_addr: standardize_address(self.seller.as_str()),
                price: self.price.parse().unwrap(),
                royalties: self.royalties.parse().unwrap(),
                commission: self.commission.parse().unwrap(),
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                activity_timestamp: time_now,
                activity_tx_version: tx_version,
                activity_event_idx: event_idx,
                activity_type: ActivityType::CollectionBidFilled as i32,
            },
        )
    }
}

impl CollectionBidCancelledEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (CollectionBid, Activity) {
        (
            CollectionBid {
                bid_obj_addr: standardize_address(self.collection_offer.as_str()),
                collection_addr: self.collection_metadata.get_collection_addr().clone(),
                collection_creator_addr: standardize_address(
                    self.collection_metadata.creator_address.as_str(),
                ),
                collection_name: self.collection_metadata.collection_name.clone(),
                nft_standard: self.collection_metadata.get_nft_standard(),
                marketplace_addr: marketplace_addr.clone(),
                total_nft_amount: 0,
                buyer_addr: standardize_address(self.purchaser.as_str()),
                price: self.price.parse().unwrap(),
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                order_placed_timestamp: 0,
                order_placed_tx_version: 0,
                order_placed_event_idx: 0,
                latest_order_filled_timestamp: 0,
                latest_order_filled_tx_version: 0,
                latest_order_filled_event_idx: 0,
                order_cancelled_timestamp: get_unix_timestamp_in_secs(),
                order_cancelled_tx_version: tx_version,
                order_cancelled_event_idx: event_idx,
                order_status: OrderStatus::Cancelled as i32,
                order_expiration_timestamp: 0,
            },
            Activity {
                nft_id: "".to_string(),
                nft_name: "".to_string(),
                collection_addr: self.collection_metadata.get_collection_addr().clone(),
                collection_creator_addr: standardize_address(
                    self.collection_metadata.creator_address.as_str(),
                ),
                collection_name: self.collection_metadata.collection_name.clone(),
                nft_standard: self.collection_metadata.get_nft_standard(),
                marketplace_addr,
                buyer_addr: standardize_address(self.purchaser.as_str()),
                seller_addr: "".to_string(),
                price: self.price.parse().unwrap(),
                royalties: 0,
                commission: 0,
                payment_token: APT_COIN.to_string(),
                payment_token_type: PaymentTokenType::Coin as i32,
                activity_timestamp: get_unix_timestamp_in_secs(),
                activity_tx_version: tx_version,
                activity_event_idx: event_idx,
                activity_type: ActivityType::CollectionBidCancelled as i32,
            },
        )
    }
}
