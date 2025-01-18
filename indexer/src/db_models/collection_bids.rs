use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::schema::collection_bids;

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
