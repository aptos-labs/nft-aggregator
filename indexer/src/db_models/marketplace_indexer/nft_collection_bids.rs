use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

use crate::marketplace_schema::nft_collection_bids;

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = nft_collection_bids)]
/// Database representation of a package upgrade change
pub struct NftCollectionBid {
    pub bid_obj_addr: String,
    pub collection_addr: String,
    pub nft_standard: i32,
    pub marketplace_addr: String,
    pub buyer_addr: String,
    pub price: i64,
    pub payment_token: String,
    pub payment_token_type: i32,
    pub create_timestamp: i64,
    pub last_update_timestamp: i64,
    pub last_update_event_idx: i64,
    pub order_status: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionMetadataOnChain {
    creator_address: String,
    collection_name: String,
    collection: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenMetadataOnChain {
    creator_address: String,
    collection_name: String,
    collection: Option<String>,
    token_name: String,
    token: Option<String>,
    property_version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionOfferCanceledEventOnChain {
    pub collection_offer: String,
    pub purchaser: String,
    pub price: u64,
    pub remaining_token_amount: u64,
    pub collection_metadata: CollectionMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionOfferFilledEventOnChain {
    pub collection_offer: String,
    pub purchaser: String,
    pub seller: String,
    pub price: u64,
    pub royalties: u64,
    pub commission: u64,
    pub token_metadata: TokenMetadataOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionOfferPlacedEventOnChain {
    pub collection_offer: String,
    pub purchaser: String,
    pub price: u64,
    pub token_amount: u64,
    pub collection_metadata: CollectionMetadataOnChain,
}

impl PackageUpgradeChangeOnChain {
    pub fn to_db_package_upgrade(
        &self,
        tx_version: i64,
        package_addr: String,
    ) -> Vec<PackageUpgrade> {
        self.packages
            .iter()
            .map(|package| PackageUpgrade {
                package_addr: package_addr.clone(),
                package_name: package.name.clone(),
                upgrade_number: package.upgrade_number.parse().unwrap(),
                upgrade_policy: package.upgrade_policy.policy,
                package_manifest: package.manifest.clone(),
                source_digest: package.source_digest.clone(),
                tx_version,
            })
            .collect()
    }
}
