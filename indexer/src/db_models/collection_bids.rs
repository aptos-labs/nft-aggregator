use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::{
    schema::{collection_bids, filled_collection_bids},
    utils::time_utils::get_unix_timestamp_in_secs,
};

use super::shared::{
    CollectionMetadataOnChain, OrderStatus, PaymentTokenType, TokenMetadataOnChain, APT_COIN,
};

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = collection_bids)]
/// Database representation of a collection bid
pub struct CollectionBid {
    pub bid_obj_addr: String,
    pub collection_addr: String,
    pub collection_creator_addr: String,
    pub collection_name: String,
    pub nft_standard: i32,
    pub marketplace_addr: String,
    pub buyer_addr: String,
    pub total_nft_amount: i64,
    pub price: i64,
    pub payment_token: String,
    pub payment_token_type: i32,
    pub order_placed_timestamp: i64,
    pub order_placed_tx_version: i64,
    pub order_placed_event_idx: i64,
    pub latest_order_filled_timestamp: i64,
    pub latest_order_filled_tx_version: i64,
    pub latest_order_filled_event_idx: i64,
    pub order_cancelled_timestamp: i64,
    pub order_cancelled_tx_version: i64,
    pub order_cancelled_event_idx: i64,
    pub order_status: i32,
    pub order_expiration_timestamp: i64,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = filled_collection_bids)]
/// Database representation of a filled collection bid
pub struct FilledCollectionBid {
    pub bid_obj_addr: String,
    pub nft_id: String,
    pub nft_name: String,
    pub seller_addr: String,
    pub price: i64,
    pub royalties: i64,
    pub commission: i64,
    pub order_filled_timestamp: i64,
    pub order_filled_tx_version: i64,
    pub order_filled_event_idx: i64,
}

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
    ) -> CollectionBid {
        CollectionBid {
            bid_obj_addr: standardize_address(self.collection_offer.as_str()),
            collection_addr: self.collection_metadata.get_collection_addr().clone(),
            collection_creator_addr: standardize_address(
                self.collection_metadata.creator_address.as_str(),
            ),
            collection_name: self.collection_metadata.collection_name.clone(),
            nft_standard: self.collection_metadata.get_nft_standard(),
            marketplace_addr,
            total_nft_amount: self.token_amount.parse().unwrap(),
            buyer_addr: standardize_address(self.purchaser.as_str()),
            price: self.price.parse().unwrap(),
            payment_token: APT_COIN.to_string(),
            payment_token_type: PaymentTokenType::Coin as i32,
            order_placed_timestamp: get_unix_timestamp_in_secs(),
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

impl CollectionBidFilledEventOnChain {
    pub fn to_db_collection_bid_and_filled_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> (CollectionBid, FilledCollectionBid) {
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
                marketplace_addr,
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
                // will calculate status later in the sql query
                // status is only filled when remaining_nft_amount is 0
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
        )
    }
}

impl CollectionBidCancelledEventOnChain {
    pub fn to_db_collection_bid(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> CollectionBid {
        CollectionBid {
            bid_obj_addr: standardize_address(self.collection_offer.as_str()),
            collection_addr: self.collection_metadata.get_collection_addr().clone(),
            collection_creator_addr: standardize_address(
                self.collection_metadata.creator_address.as_str(),
            ),
            collection_name: self.collection_metadata.collection_name.clone(),
            nft_standard: self.collection_metadata.get_nft_standard(),
            marketplace_addr,
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
        }
    }
}
