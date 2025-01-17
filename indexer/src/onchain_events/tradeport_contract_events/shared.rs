use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NftV1TokenDataId {
    pub creator: String,
    pub collection: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NftV1TokenId {
    /// the id to the common token data shared by token with different property_version
    pub token_data_id: NftV1TokenDataId,
    /// The version of the property map; when a fungible token is mutated, a new property version is created and assigned to the token to make it an NFT
    pub property_version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NftV1CollectionId {
    pub collection_creator: String,
    pub collection_name: String,
}

// if the an user sell a nft, buy it back, then sell it again, 2 orders will have the same order_id
// i can't think of a workaround here, because if we add other element (e.g. order placed tx version) to the order_id, we cannot retrieve the order id when the order is filled later
// we just hope no one is buy/sell v1 anymore
pub fn generate_ask_order_id_for_nft_v1(owner: String, token_id: NftV1TokenId) -> String {
    let order_id = format!(
        "{}_{}_{}_{}_{}",
        token_id.token_data_id.creator,
        token_id.token_data_id.collection,
        token_id.token_data_id.name,
        token_id.property_version,
        owner
    );

    let hash = blake3::hash(order_id.as_bytes());
    hex::encode(hash.as_bytes())
}

pub fn generate_bid_order_id_for_nft_v1(nonce: String) -> String {
    let order_id = format!("tradeport_v1_bid_order_{}", nonce);

    let hash = blake3::hash(order_id.as_bytes());
    hex::encode(hash.as_bytes())
}

pub fn generate_collection_bid_order_id_for_nft_v1(nonce: String) -> String {
    let order_id = format!("tradeport_v1_collection_bid_order__{}", nonce);

    let hash = blake3::hash(order_id.as_bytes());
    hex::encode(hash.as_bytes())
}
