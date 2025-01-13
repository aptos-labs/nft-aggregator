use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::{schema::nft_asks, utils::time_utils::get_unix_timestamp_in_secs};

use super::shared::{
    AskOrderType, NftV1TokenId, OrderStatus, PaymentTokenType, TokenMetadataOnChain, APT_COIN,
};

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = nft_asks)]
/// Database representation of a nft ask
pub struct NftAsk {
    pub ask_obj_addr: String,
    pub nft_id: String,
    pub nft_name: String,
    pub collection_addr: String,
    pub collection_creator_addr: String,
    pub collection_name: String,
    pub nft_standard: i32,
    pub marketplace_addr: String,
    pub buyer_addr: String,
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
    pub order_type: i32,
}

fn convert_on_chain_order_type_to_db_order_type(order_type: &str) -> AskOrderType {
    match order_type {
        "fixed price" => AskOrderType::FixedPrice,
        "auction" => AskOrderType::Auction,
        _ => panic!("Invalid ask order type"),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AskPlacedEventOnChain {
    #[serde(rename = "type")]
    pub order_type: String,
    pub listing: String,
    pub seller: String,
    pub price: String,
    pub token_metadata: TokenMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AskFilledEventOnChain {
    #[serde(rename = "type")]
    pub order_type: String,
    pub listing: String,
    pub seller: String,
    pub purchaser: String,
    pub price: String,
    pub commission: String,
    pub royalties: String,
    pub token_metadata: TokenMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AskCancelledEventOnChain {
    #[serde(rename = "type")]
    pub order_type: String,
    pub listing: String,
    pub seller: String,
    pub price: String,
    pub token_metadata: TokenMetadataOnChain,
}

// Tradeport v1 InsertListingEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1AskPlacedEventOnChain {
    pub timestamp: String,
    pub token_id: NftV1TokenId,
    pub price: String,
    pub owner: String,
}

// Tradeport v1 UpdateListingEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1AskUpdatedEventOnChain {
    pub timestamp: String,
    pub token_id: NftV1TokenId,
    pub price: String,
    pub old_price: String,
    pub owner: String,
}

// Tradeport v1 DeleteListingEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1AskCancelledEventOnChain {
    pub timestamp: String,
    pub token_id: NftV1TokenId,
    pub price: String,
    pub owner: String,
}

// Tradeport v1 BuyEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1AskFilledEventOnChain {
    pub timestamp: String,
    pub token_id: NftV1TokenId,
    pub price: String,
    pub owner: String,
    pub buyer: String,
}

impl AskPlacedEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: standardize_address(self.listing.as_str()),
            nft_id: self.token_metadata.get_id(),
            nft_name: self.token_metadata.token_name.clone(),
            collection_addr: self.token_metadata.get_collection_addr().clone(),
            collection_creator_addr: standardize_address(
                self.token_metadata.creator_address.as_str(),
            ),
            collection_name: self.token_metadata.collection_name.clone(),
            nft_standard: self.token_metadata.get_nft_standard(),
            marketplace_addr,
            buyer_addr: "".to_string(),
            seller_addr: standardize_address(self.seller.as_str()),
            price: self.price.parse().unwrap(),
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
            order_type: convert_on_chain_order_type_to_db_order_type(&self.order_type) as i32,
        }
    }
}

impl AskFilledEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: standardize_address(self.listing.as_str()),
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
            commission: self.commission.parse().unwrap(),
            royalties: self.royalties.parse().unwrap(),
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
            order_type: convert_on_chain_order_type_to_db_order_type(&self.order_type) as i32,
        }
    }
}

impl AskCancelledEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: standardize_address(self.listing.as_str()),
            nft_id: self.token_metadata.get_id(),
            nft_name: self.token_metadata.token_name.clone(),
            collection_addr: self.token_metadata.get_collection_addr().clone(),
            collection_creator_addr: standardize_address(
                self.token_metadata.creator_address.as_str(),
            ),
            collection_name: self.token_metadata.collection_name.clone(),
            nft_standard: self.token_metadata.get_nft_standard(),
            marketplace_addr,
            buyer_addr: "".to_string(),
            seller_addr: standardize_address(self.seller.as_str()),
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
            order_cancelled_timestamp: get_unix_timestamp_in_secs(),
            order_cancelled_tx_version: tx_version,
            order_cancelled_event_idx: event_idx,
            order_status: OrderStatus::Cancelled as i32,
            order_type: convert_on_chain_order_type_to_db_order_type(&self.order_type) as i32,
        }
    }
}
