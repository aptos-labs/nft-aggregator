use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NftV1CollectionId {
    pub collection_creator: String,
    pub collection_name: String,
}
