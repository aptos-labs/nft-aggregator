-- Your SQL goes here
CREATE INDEX idx_nft_bids_nft_id ON nft_bids (nft_id);

CREATE INDEX idx_nft_bids_collection_addr ON nft_bids (collection_addr);

CREATE INDEX idx_nft_bids_buyer_addr ON nft_bids (buyer_addr);

CREATE INDEX idx_nft_bids_marketplace_addr ON nft_bids (marketplace_addr);

CREATE INDEX idx_nft_bids_order_type ON nft_bids (order_type);