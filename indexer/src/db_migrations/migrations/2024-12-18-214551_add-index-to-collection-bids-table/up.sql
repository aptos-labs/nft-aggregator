-- Your SQL goes here
CREATE INDEX idx_collection_bids_collection_addr ON collection_bids (collection_addr);

CREATE INDEX idx_collection_bids_collection_creator_addr ON collection_bids (collection_creator_addr);

CREATE INDEX idx_collection_bids_collection_name ON collection_bids (collection_name);

CREATE INDEX idx_collection_bids_buyer_addr ON collection_bids (buyer_addr);

CREATE INDEX idx_collection_bids_marketplace_addr ON collection_bids (marketplace_addr);