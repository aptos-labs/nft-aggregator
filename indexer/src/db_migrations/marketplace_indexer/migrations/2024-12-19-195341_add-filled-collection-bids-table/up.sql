-- This table should be used together with the collection_bids table to store filled collection bids
-- This table is backfill safe, i.e. you can re-index without dropping the table
-- Your SQL goes here
CREATE TABLE
    filled_collection_bids (
        bid_obj_addr VARCHAR(300) NOT NULL,
        -- For v1 NFTs, this is property_version, for v2 NFTs, this is nft_addr
        -- For collection bid, this only gets set after order is filled
        nft_id VARCHAR(300) NOT NULL,
        nft_name VARCHAR(300) NOT NULL,
        -- price in on-chain unit, for APT it's oct
        price BIGINT NOT NULL,
        -- in on-chain unit, for APT it's oct
        royalties BIGINT NOT NULL,
        -- in on-chain unit, for APT it's oct
        commission BIGINT NOT NULL,
        order_filled_timestamp BIGINT NOT NULL,
        order_filled_tx_version BIGINT NOT NULL,
        order_filled_event_idx BIGINT NOT NULL,
        PRIMARY KEY (bid_obj_addr, nft_id, nft_name),
        FOREIGN KEY (bid_obj_addr) REFERENCES collection_bids (bid_obj_addr)
    );