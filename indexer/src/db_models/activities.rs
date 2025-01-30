use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::schema::activities;

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = activities)]
/// Database representation of an activity
pub struct Activity {
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
    pub activity_timestamp: i64,
    pub activity_tx_version: i64,
    pub activity_event_idx: i64,
    pub activity_type: i32,
}
