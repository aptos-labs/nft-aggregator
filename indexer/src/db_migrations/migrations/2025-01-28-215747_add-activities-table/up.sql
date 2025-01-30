-- This table is backfill safe, i.e. you can re-index without dropping the table
CREATE TABLE
    activities (
        -- For v1 NFTs, this is property_version, for v2 NFTs, this is nft_addr
        nft_id VARCHAR(300) NOT NULL,
        nft_name VARCHAR(300) NOT NULL,
        -- For v2 NFTs, we use collection_addr to identify the collection
        collection_addr VARCHAR(300) NOT NULL,
        -- For v1 NFTs, we use creator_addr + name to identify the collection
        collection_creator_addr VARCHAR(300) NOT NULL,
        collection_name VARCHAR(300) NOT NULL,
        -- 1 is token v1, 2 is token v2
        nft_standard INT NOT NULL,
        marketplace_addr VARCHAR(300) NOT NULL,
        buyer_addr VARCHAR(300) NOT NULL,
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
        -- 1 is collection bid placed
        -- 2 is collection bid filled
        -- 3 is collection bid cancelled
        -- 4 is nft bid placed
        -- 5 is nft bid filled
        -- 6 is nft bid cancelled
        -- 7 is nft ask placed
        -- 8 is nft ask filled
        -- 9 is nft ask cancelled
        activity_type INT NOT NULL,
        activity_tx_version BIGINT NOT NULL,
        activity_timestamp BIGINT NOT NULL,
        activity_event_idx BIGINT NOT NULL,
        CHECK (nft_standard IN (1, 2)),
        CHECK (payment_token_type IN (1, 2)),
        CHECK (activity_type IN (1, 2, 3, 4, 5, 6, 7, 8, 9)),
        PRIMARY KEY (activity_tx_version, activity_event_idx)
    );