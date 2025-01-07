use aptos_indexer_processor_sdk::utils::convert::standardize_address;
use serde::{Deserialize, Serialize};

pub const APT_COIN: &str = "0x1::aptos_coin::AptosCoin";
pub const APT_FA: &str = "0xa";

pub enum NFTStandard {
    V1 = 1,
    V2 = 2,
}

pub enum PaymentTokenType {
    Coin = 1,
    FA = 2,
}

pub enum OrderStatus {
    Open = 1,
    Filled = 2,
    Cancelled = 3,
}

pub enum AskOrderType {
    FixedPrice = 1,
    Auction = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionMetadataOnChain {
    pub creator_address: String,
    pub collection_name: String,
    pub collection: MoveOptionObject,
}

impl CollectionMetadataOnChain {
    pub fn is_v1(&self) -> bool {
        self.collection.vec.len() == 0
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
        self.collection.vec.len() == 0
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NftV1TokenDataId {
    creator: String,
    collection: String,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NftV1TokenId {
    /// the id to the common token data shared by token with different property_version
    pub token_data_id: NftV1TokenDataId,
    /// The version of the property map; when a fungible token is mutated, a new property version is created and assigned to the token to make it an NFT
    pub property_version: u64,
}
