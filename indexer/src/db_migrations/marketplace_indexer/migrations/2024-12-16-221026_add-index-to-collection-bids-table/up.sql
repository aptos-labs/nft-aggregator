-- Your SQL goes here
CREATE INDEX idx_nft_collection_bids_collection_addr ON nft_bids (collection_addr);

CREATE INDEX idx_nft_collection_bids_buyer_addr ON nft_bids (buyer_addr);

CREATE INDEX idx_nft_collection_bids_marketplace_addr ON nft_bids (marketplace_addr);