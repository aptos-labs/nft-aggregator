use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::{
    db_models::nft_asks::NftAsk,
    utils::aptos_utils::{AskOrderType, NFTStandard, OrderStatus, PaymentTokenType, APT_COIN},
};

use super::shared::{generate_ask_order_id_for_nft_v1, NftV1TokenId};

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

// Tradeport v1 BuyEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1AskFilledEventOnChain {
    pub timestamp: String,
    pub token_id: NftV1TokenId,
    pub price: String,
    pub owner: String,
    pub buyer: String,
}

// Tradeport v1 DeleteListingEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1AskCancelledEventOnChain {
    pub timestamp: String,
    pub token_id: NftV1TokenId,
    pub price: String,
    pub owner: String,
}

impl TradeportV1AskPlacedEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: generate_ask_order_id_for_nft_v1(
                self.owner.clone(),
                self.token_id.clone(),
            ),
            nft_id: self.token_id.property_version.clone(),
            nft_name: self.token_id.token_data_id.name.clone(),
            collection_addr: "".to_string(),
            collection_creator_addr: standardize_address(
                self.token_id.token_data_id.creator.as_str(),
            ),
            collection_name: self.token_id.token_data_id.collection.clone(),
            nft_standard: NFTStandard::V1 as i32,
            marketplace_addr,
            buyer_addr: "".to_string(),
            seller_addr: standardize_address(self.owner.as_str()),
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
            order_type: AskOrderType::FixedPrice as i32,
        }
    }
}

impl TradeportV1AskUpdatedEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: generate_ask_order_id_for_nft_v1(
                self.owner.clone(),
                self.token_id.clone(),
            ),
            nft_id: "".to_string(),
            nft_name: "".to_string(),
            collection_addr: "".to_string(),
            collection_creator_addr: "".to_string(),
            collection_name: "".to_string(),
            nft_standard: 1,
            marketplace_addr,
            buyer_addr: "".to_string(),
            seller_addr: standardize_address(self.owner.as_str()),
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
            order_status: OrderStatus::Open as i32,
            order_type: AskOrderType::FixedPrice as i32,
        }
    }
}

impl TradeportV1AskFilledEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: generate_ask_order_id_for_nft_v1(
                self.owner.clone(),
                self.token_id.clone(),
            ),
            nft_id: "".to_string(),
            nft_name: "".to_string(),
            collection_addr: "".to_string(),
            collection_creator_addr: "".to_string(),
            collection_name: "".to_string(),
            nft_standard: 1,
            marketplace_addr,
            buyer_addr: standardize_address(self.buyer.as_str()),
            seller_addr: standardize_address(self.owner.as_str()),
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
            order_type: AskOrderType::FixedPrice as i32,
        }
    }
}

impl TradeportV1AskCancelledEventOnChain {
    pub fn to_db_nft_ask(
        &self,
        marketplace_addr: String,
        tx_version: i64,
        event_idx: i64,
    ) -> NftAsk {
        NftAsk {
            ask_obj_addr: generate_ask_order_id_for_nft_v1(
                self.owner.clone(),
                self.token_id.clone(),
            ),
            nft_id: "".to_string(),
            nft_name: "".to_string(),
            collection_addr: "".to_string(),
            collection_creator_addr: "".to_string(),
            collection_name: "".to_string(),
            nft_standard: 1,
            marketplace_addr,
            buyer_addr: "".to_string(),
            seller_addr: standardize_address(self.owner.as_str()),
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
            order_type: AskOrderType::FixedPrice as i32,
        }
    }
}
