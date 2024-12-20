-- Your SQL goes here
CREATE INDEX idx_nft_bids_nft_id ON nft_bids (nft_id);

CREATE INDEX idx_nft_bids_nft_name ON nft_bids (nft_name);

CREATE INDEX idx_nft_bids_collection_addr ON nft_bids (collection_addr);

CREATE INDEX idx_nft_bids_collection_creator_addr ON nft_bids (collection_creator_addr);

CREATE INDEX idx_nft_bids_collection_name ON nft_bids (collection_name);

CREATE INDEX idx_nft_bids_buyer_addr ON nft_bids (buyer_addr);

CREATE INDEX idx_nft_bids_marketplace_addr ON nft_bids (marketplace_addr);