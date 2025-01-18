use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::schema::filled_collection_bids;

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
