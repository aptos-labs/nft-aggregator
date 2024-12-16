-- Your SQL goes here
CREATE INDEX idx_nft_asks_nft_id ON nft_asks (nft_id);

CREATE INDEX idx_nft_asks_collection_addr ON nft_asks (collection_addr);

CREATE INDEX idx_nft_asks_seller_addr ON nft_asks (seller_addr);

CREATE INDEX idx_nft_asks_marketplace_addr ON nft_asks (marketplace_addr);

CREATE INDEX idx_nft_asks_order_type ON nft_asks (order_type);