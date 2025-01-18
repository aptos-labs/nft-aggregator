use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

use crate::utils::aptos_utils::NFTStandard;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionMetadataOnChain {
    pub creator_address: String,
    pub collection_name: String,
    pub collection: MoveOptionObject,
}

impl CollectionMetadataOnChain {
    pub fn is_v1(&self) -> bool {
        self.collection.vec.is_empty()
    }

    pub fn get_nft_standard(&self) -> i32 {
        if self.is_v1() {
            NFTStandard::V1 as i32
        } else {
            NFTStandard::V2 as i32
        }
    }

    pub fn get_collection_addr(&self) -> String {
        if self.is_v1() {
            "".to_string()
        } else {
            standardize_address(self.collection.vec[0].clone().inner.as_str())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoveObject {
    pub inner: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoveOptionObject {
    pub vec: Vec<MoveObject>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoveOptionU64 {
    pub vec: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenMetadataOnChain {
    pub creator_address: String,
    pub collection_name: String,
    pub collection: MoveOptionObject,
    pub token_name: String,
    pub token: MoveOptionObject,
    pub property_version: MoveOptionU64,
}

impl TokenMetadataOnChain {
    pub fn is_v1(&self) -> bool {
        self.collection.vec.is_empty()
    }

    pub fn get_nft_standard(&self) -> i32 {
        if self.is_v1() {
            NFTStandard::V1 as i32
        } else {
            NFTStandard::V2 as i32
        }
    }

    pub fn get_id(&self) -> String {
        if self.is_v1() {
            self.property_version.vec[0].clone()
        } else {
            standardize_address(self.token.vec[0].clone().inner.as_str())
        }
    }

    pub fn get_collection_addr(&self) -> String {
        if self.is_v1() {
            "".to_string()
        } else {
            standardize_address(self.collection.vec[0].clone().inner.as_str())
        }
    }

    // for v1 nft, identifier is nft_name_property_version
    // for v2 nft, identifier is nft obj addr
    pub fn get_nft_identifier_for_filled_order(&self) -> String {
        if self.is_v1() {
            format!(
                "{}_{}",
                self.token_name.clone(),
                self.property_version.vec[0].clone()
            )
        } else {
            standardize_address(self.token.vec[0].clone().inner.as_str())
        }
    }
}
