use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::schema::nft_asks;

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
