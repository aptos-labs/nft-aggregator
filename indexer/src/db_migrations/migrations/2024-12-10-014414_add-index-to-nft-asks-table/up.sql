-- Your SQL goes here
CREATE INDEX idx_nft_asks_nft_id ON nft_asks (nft_id);

CREATE INDEX idx_nft_asks_nft_name ON nft_asks (nft_name);

CREATE INDEX idx_nft_asks_collection_addr ON nft_asks (collection_addr);

CREATE INDEX idx_nft_asks_collection_creator_addr ON nft_asks (collection_addr);

CREATE INDEX idx_nft_asks_collection_name ON nft_asks (collection_addr);

CREATE INDEX idx_nft_asks_buyer_addr ON nft_asks (buyer_addr);

CREATE INDEX idx_nft_asks_seller_addr ON nft_asks (seller_addr);

CREATE INDEX idx_nft_asks_marketplace_addr ON nft_asks (marketplace_addr);