use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::{db_models::nft_bids::NftBid, utils::time_utils::get_unix_timestamp_in_secs};

use super::shared::NftV1TokenId;

// Tradeport v1 InsertTokenBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BidPlacedEventOnChain {
    pub timestamp: String,
    pub nonce: String,
    pub bid_buyer: String,
    pub token_id: NftV1TokenId,
    pub price: String,
}

// Tradeport v1 AcceptTokenBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeportV1BidFilledEventOnChain {
    pub timestamp: String,
    pub nonce: String,
    pub bid_buyer: String,
    pub bid_seller: String,
    pub token_id: NftV1TokenId,
    pub price: String,
}

// Tradeport v1 DeleteTokenBidEvent
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BidCancelledEventOnChain {
    pub timestamp: String,
    pub nonce: String,
    pub bid_buyer: String,
    pub token_id: NftV1TokenId,
    pub price: String,
}

// impl BidPlacedEventOnChain {
//     pub fn to_db_nft_bid(
//         &self,
//         marketplace_addr: String,
//         tx_version: i64,
//         event_idx: i64,
//     ) -> NftBid {
//         NftBid {
//             bid_obj_addr: standardize_address(self.token_offer.as_str()),
//             nft_id: self.token_metadata.get_id(),
//             nft_name: self.token_metadata.token_name.clone(),
//             collection_addr: self.token_metadata.get_collection_addr().clone(),
//             collection_creator_addr: standardize_address(
//                 self.token_metadata.creator_address.as_str(),
//             ),
//             collection_name: self.token_metadata.collection_name.clone(),
//             nft_standard: self.token_metadata.get_nft_standard(),
//             marketplace_addr,
//             buyer_addr: standardize_address(self.purchaser.as_str()),
//             seller_addr: "".to_string(),
//             price: self.price.parse().unwrap(),
//             commission: 0,
//             royalties: 0,
//             payment_token: APT_COIN.to_string(),
//             payment_token_type: PaymentTokenType::Coin as i32,
//             order_placed_timestamp: get_unix_timestamp_in_secs(),
//             order_placed_tx_version: tx_version,
//             order_placed_event_idx: event_idx,
//             order_filled_timestamp: 0,
//             order_filled_tx_version: 0,
//             order_filled_event_idx: 0,
//             order_cancelled_timestamp: 0,
//             order_cancelled_tx_version: 0,
//             order_cancelled_event_idx: 0,
//             order_status: OrderStatus::Open as i32,
//             order_expiration_timestamp: 0,
//         }
//     }
// }

// impl BidFilledEventOnChain {
//     pub fn to_db_nft_bid(
//         &self,
//         marketplace_addr: String,
//         tx_version: i64,
//         event_idx: i64,
//     ) -> NftBid {
//         NftBid {
//             bid_obj_addr: standardize_address(self.token_offer.as_str()),
//             nft_id: self.token_metadata.get_id(),
//             nft_name: self.token_metadata.token_name.clone(),
//             collection_addr: self.token_metadata.get_collection_addr().clone(),
//             collection_creator_addr: standardize_address(
//                 self.token_metadata.creator_address.as_str(),
//             ),
//             collection_name: self.token_metadata.collection_name.clone(),
//             nft_standard: self.token_metadata.get_nft_standard(),
//             marketplace_addr,
//             buyer_addr: standardize_address(self.purchaser.as_str()),
//             seller_addr: standardize_address(self.seller.as_str()),
//             price: self.price.parse().unwrap(),
//             commission: self.commission.parse().unwrap(),
//             royalties: self.royalties.parse().unwrap(),
//             payment_token: APT_COIN.to_string(),
//             payment_token_type: PaymentTokenType::Coin as i32,
//             order_placed_timestamp: 0,
//             order_placed_tx_version: 0,
//             order_placed_event_idx: 0,
//             order_filled_timestamp: get_unix_timestamp_in_secs(),
//             order_filled_tx_version: tx_version,
//             order_filled_event_idx: event_idx,
//             order_cancelled_timestamp: 0,
//             order_cancelled_tx_version: 0,
//             order_cancelled_event_idx: 0,
//             order_status: OrderStatus::Filled as i32,
//             order_expiration_timestamp: 0,
//         }
//     }
// }

// impl BidCancelledEventOnChain {
//     pub fn to_db_nft_bid(
//         &self,
//         marketplace_addr: String,
//         tx_version: i64,
//         event_idx: i64,
//     ) -> NftBid {
//         NftBid {
//             bid_obj_addr: standardize_address(self.token_offer.as_str()),
//             nft_id: self.token_metadata.get_id(),
//             nft_name: self.token_metadata.token_name.clone(),
//             collection_addr: self.token_metadata.get_collection_addr().clone(),
//             collection_creator_addr: standardize_address(
//                 self.token_metadata.creator_address.as_str(),
//             ),
//             collection_name: self.token_metadata.collection_name.clone(),
//             nft_standard: self.token_metadata.get_nft_standard(),
//             marketplace_addr,
//             buyer_addr: standardize_address(self.purchaser.as_str()),
//             seller_addr: "".to_string(),
//             price: self.price.parse().unwrap(),
//             commission: 0,
//             royalties: 0,
//             payment_token: APT_COIN.to_string(),
//             payment_token_type: PaymentTokenType::Coin as i32,
//             order_placed_timestamp: 0,
//             order_placed_tx_version: 0,
//             order_placed_event_idx: 0,
//             order_filled_timestamp: 0,
//             order_filled_tx_version: 0,
//             order_filled_event_idx: 0,
//             order_cancelled_timestamp: get_unix_timestamp_in_secs(),
//             order_cancelled_tx_version: tx_version,
//             order_cancelled_event_idx: event_idx,
//             order_status: OrderStatus::Cancelled as i32,
//             order_expiration_timestamp: 0,
//         }
//     }
// }
