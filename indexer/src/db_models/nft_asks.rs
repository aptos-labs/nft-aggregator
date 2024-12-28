use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::{schema::nft_asks, utils::utils::get_unix_timestamp_in_secs};

use super::shared::{OrderStatus, PaymentTokenType, TokenMetadataOnChain, APT_COIN};

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = nft_asks)]
/// Database representation of a nft ask
pub struct NftAsk {
    pub ask_obj_addr: String,
    pub nft_id: String,
    pub nft_name: String,
    pub collection_addr: Option<String>,
    pub collection_creator_addr: String,
    pub collection_name: String,
    pub nft_standard: i32,
    pub marketplace_addr: String,
    pub seller_addr: String,
    pub price: i64,
    pub royalties: i64,
    pub commission: i64,
    pub payment_token: String,
    pub payment_token_type: i32,
    pub order_placed_timestamp: i64,
    pub order_placed_tx_version: i64,
    pub order_placed_event_idx: i64,
    pub order_filled_timestamp: i64,
    pub order_filled_tx_version: i64,
    pub order_filled_event_idx: i64,
    pub order_cancelled_timestamp: i64,
    pub order_cancelled_tx_version: i64,
    pub order_cancelled_event_idx: i64,
    pub order_status: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AskOrderPlacedEventOnChain {
    #[serde(rename = "type")]
    pub order_type: String,
    pub listing: String,
    pub seller: String,
    pub price: u64,
    pub token_metadata: TokenMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AskOrderFilledEventOnChain {
    #[serde(rename = "type")]
    pub order_type: String,
    pub listing: String,
    pub seller: String,
    pub purchaser: String,
    pub price: u64,
    pub commission: u64,
    pub royalties: u64,
    pub token_metadata: TokenMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AskOrderCancelledEventOnChain {
    #[serde(rename = "type")]
    pub order_type: String,
    pub listing: String,
    pub seller: String,
    pub price: u64,
    pub token_metadata: TokenMetadataOnChain,
}

impl AskOrderPlacedEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: self.listing.clone(),
            nft_id: self.token_metadata.get_id(),
            nft_name: self.token_metadata.token_name.clone(),
            collection_addr: self.token_metadata.collection.clone(),
            collection_creator_addr: self.token_metadata.creator_address.clone(),
            collection_name: self.token_metadata.collection_name.clone(),
            nft_standard: self.token_metadata.get_nft_standard(),
            marketplace_addr,
            seller_addr: self.seller.clone(),
            price: self.price as i64,
            commission: 0,
            royalties: 0,
            payment_token: APT_COIN.to_string(),
            payment_token_type: PaymentTokenType::Coin as i32,
            order_placed_timestamp: get_unix_timestamp_in_secs(),
            order_placed_tx_version: tx_version,
            order_placed_event_idx: event_idx,
            order_filled_timestamp: 0,
            order_filled_tx_version: 0,
            order_filled_event_idx: 0,
            order_cancelled_timestamp: 0,
            order_cancelled_tx_version: 0,
            order_cancelled_event_idx: 0,
            order_status: OrderStatus::Open as i32,
        }
    }
}

impl AskOrderFilledEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: self.listing.clone(),
            nft_id: self.token_metadata.get_id(),
            nft_name: self.token_metadata.token_name.clone(),
            collection_addr: self.token_metadata.collection.clone(),
            collection_creator_addr: self.token_metadata.creator_address.clone(),
            collection_name: self.token_metadata.collection_name.clone(),
            nft_standard: self.token_metadata.get_nft_standard(),
            marketplace_addr,
            seller_addr: self.seller.clone(),
            price: self.price as i64,
            commission: self.commission as i64,
            royalties: self.royalties as i64,
            payment_token: APT_COIN.to_string(),
            payment_token_type: PaymentTokenType::Coin as i32,
            order_placed_timestamp: 0,
            order_placed_tx_version: 0,
            order_placed_event_idx: 0,
            order_filled_timestamp: get_unix_timestamp_in_secs(),
            order_filled_tx_version: tx_version,
            order_filled_event_idx: event_idx,
            order_cancelled_timestamp: 0,
            order_cancelled_tx_version: 0,
            order_cancelled_event_idx: 0,
            order_status: OrderStatus::Filled as i32,
        }
    }
}

impl AskOrderCancelledEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: self.listing.clone(),
            nft_id: self.token_metadata.get_id(),
            nft_name: self.token_metadata.token_name.clone(),
            collection_addr: self.token_metadata.collection.clone(),
            collection_creator_addr: self.token_metadata.creator_address.clone(),
            collection_name: self.token_metadata.collection_name.clone(),
            nft_standard: self.token_metadata.get_nft_standard(),
            marketplace_addr,
            seller_addr: self.seller.clone(),
            price: self.price as i64,
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
            order_cancelled_timestamp: get_unix_timestamp_in_secs(),
            order_cancelled_tx_version: tx_version,
            order_cancelled_event_idx: event_idx,
            order_status: OrderStatus::Cancelled as i32,
        }
    }
}
