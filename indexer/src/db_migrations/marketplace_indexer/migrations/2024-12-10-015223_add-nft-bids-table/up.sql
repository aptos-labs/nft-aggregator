-- Your SQL goes here
CREATE TABLE
    nft_bids (
        bid_obj_addr VARCHAR(300) PRIMARY KEY,
        -- For v1 NFTs, this is id, for v2 NFTs, this is nft_addr
        nft_id VARCHAR(300) NOT NULL,
        collection_addr VARCHAR(300) NOT NULL,
        -- 1 is token v1, 2 is token v2
        nft_standard INT NOT NULL,
        marketplace_addr VARCHAR(300) NOT NULL,
        buyer_addr VARCHAR(300) NOT NULL,
        -- price in on-chain unit, for APT it's oct
        price BIGINT NOT NULL,
        -- for coin APT, this is 0x0000000000000000000000000000000000000000000000000000000000000001::aptos_coin::AptosCoin
        -- for fa APT, this is 0x000000000000000000000000000000000000000000000000000000000000000a
        payment_token VARCHAR(300) NOT NULL,
        -- 1 is coin, 2 is fa
        payment_token_type INT NOT NULL,
        -- when the bid is placed
        create_timestamp BIGINT NOT NULL,
        last_update_timestamp BIGINT NOT NULL,
        last_update_event_idx BIGINT NOT NULL,
        -- 1 is active, 2 is filled, 3 is cancelled
        order_status INT NOT NULL,
        CHECK (nft_standard IN (1, 2)),
        CHECK (payment_token_type IN (1, 2)),
        CHECK (order_status IN (1, 2, 3))
    );