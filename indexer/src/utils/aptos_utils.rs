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

pub enum ActivityType {
    CollectionBidPlaced = 1,
    CollectionBidFilled = 2,
    CollectionBidCancelled = 3,
    NFTBidPlaced = 4,
    NFTBidFilled = 5,
    NFTBidCancelled = 6,
    NFTAskPlaced = 7,
    NFTAskFilled = 8,
    NFTAskCancelled = 9,
}
