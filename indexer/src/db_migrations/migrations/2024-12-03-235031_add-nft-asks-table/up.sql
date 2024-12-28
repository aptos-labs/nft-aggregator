-- This table is backfill safe, i.e. you can re-index without dropping the table
CREATE TABLE
    nft_asks (
        ask_obj_addr VARCHAR(300) PRIMARY KEY,
        -- For v1 NFTs, this is property_version, for v2 NFTs, this is nft_addr
        nft_id VARCHAR(300) NOT NULL,
        nft_name VARCHAR(300) NOT NULL,
        -- For v2 NFTs, we use collection_addr to identify the collection
        collection_addr VARCHAR(300),
        -- For v1 NFTs, we use creator_addr + name to identify the collection
        collection_creator_addr VARCHAR(300) NOT NULL,
        collection_name VARCHAR(300) NOT NULL,
        -- 1 is token v1, 2 is token v2
        nft_standard INT NOT NULL,
        marketplace_addr VARCHAR(300) NOT NULL,
        seller_addr VARCHAR(300) NOT NULL,
        -- price in on-chain unit, for APT it's oct
        price BIGINT NOT NULL,
        -- in on-chain unit, for APT it's oct
        royalties BIGINT NOT NULL,
        -- in on-chain unit, for APT it's oct
        commission BIGINT NOT NULL,
        -- for coin APT, this is 0x1::aptos_coin::AptosCoin
        -- for fa APT, this is 0xa
        payment_token VARCHAR(300) NOT NULL,
        -- 1 is coin, 2 is fa
        payment_token_type INT NOT NULL,
        order_placed_timestamp BIGINT NOT NULL,
        order_placed_tx_version BIGINT NOT NULL,
        order_placed_event_idx BIGINT NOT NULL,
        order_filled_timestamp BIGINT NOT NULL,
        order_filled_tx_version BIGINT NOT NULL,
        order_filled_event_idx BIGINT NOT NULL,
        order_cancelled_timestamp BIGINT NOT NULL,
        order_cancelled_tx_version BIGINT NOT NULL,
        order_cancelled_event_idx BIGINT NOT NULL,
        -- 1 is active, 2 is filled, 3 is cancelled
        order_status INT NOT NULL,
        CHECK (nft_standard IN (1, 2)),
        CHECK (payment_token_type IN (1, 2)),
        CHECK (order_status IN (1, 2, 3))
    );