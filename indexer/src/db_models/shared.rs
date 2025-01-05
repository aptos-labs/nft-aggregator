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
    pub collection: Option<String>,
}

impl CollectionMetadataOnChain {
    pub fn is_v1(&self) -> bool {
        self.collection.is_none()
    }

    pub fn get_nft_standard(&self) -> i32 {
        if self.is_v1() {
            NFTStandard::V1 as i32
        } else {
            NFTStandard::V2 as i32
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenMetadataOnChain {
    pub creator_address: String,
    pub collection_name: String,
    pub collection: Option<String>,
    pub token_name: String,
    pub token: Option<String>,
    pub property_version: Option<u64>,
}

impl TokenMetadataOnChain {
    pub fn is_v1(&self) -> bool {
        self.token.is_none()
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
            self.property_version.clone().unwrap().to_string()
        } else {
            self.token.clone().unwrap()
        }
    }

    // for v1 nft, identifier is nft_name_property_version
    // for v2 nft, identifier is nft obj addr
    pub fn get_nft_identifier_for_filled_order(&self) -> String {
        if self.is_v1() {
            format!(
                "{}_{}",
                self.token_name.clone(),
                self.property_version.clone().unwrap().to_string()
            )
        } else {
            self.token.clone().unwrap()
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
