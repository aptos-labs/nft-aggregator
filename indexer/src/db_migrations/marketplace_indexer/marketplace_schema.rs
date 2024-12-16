// @generated automatically by Diesel CLI.

diesel::table! {
    contract_upgrade_ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    contract_upgrade_processor_status (processor) {
        #[max_length = 50]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    marketplace_ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    marketplace_processor_status (processor) {
        #[max_length = 50]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    module_upgrade_history (module_addr, module_name, package_name, upgrade_number) {
        #[max_length = 300]
        module_addr -> Varchar,
        #[max_length = 300]
        module_name -> Varchar,
        #[max_length = 300]
        package_name -> Varchar,
        upgrade_number -> Int8,
        module_bytecode -> Bytea,
        module_source_code -> Text,
        module_abi -> Json,
        tx_version -> Int8,
    }
}

diesel::table! {
    nft_asks (listing_obj_addr) {
        #[max_length = 300]
        listing_obj_addr -> Varchar,
        #[max_length = 300]
        nft_id -> Varchar,
        #[max_length = 300]
        collection_addr -> Varchar,
        nft_standard -> Int4,
        #[max_length = 300]
        marketplace_addr -> Varchar,
        #[max_length = 300]
        seller_addr -> Varchar,
        price -> Int8,
        #[max_length = 300]
        payment_token -> Varchar,
        payment_token_type -> Int4,
        create_timestamp -> Int8,
        last_update_timestamp -> Int8,
        last_update_event_idx -> Int8,
        order_status -> Int4,
    }
}

diesel::table! {
    nft_bids (bid_obj_addr) {
        #[max_length = 300]
        bid_obj_addr -> Varchar,
        #[max_length = 300]
        nft_id -> Varchar,
        #[max_length = 300]
        collection_addr -> Varchar,
        nft_standard -> Int4,
        #[max_length = 300]
        marketplace_addr -> Varchar,
        #[max_length = 300]
        buyer_addr -> Varchar,
        price -> Int8,
        #[max_length = 300]
        payment_token -> Varchar,
        payment_token_type -> Int4,
        create_timestamp -> Int8,
        last_update_timestamp -> Int8,
        last_update_event_idx -> Int8,
        order_status -> Int4,
    }
}

diesel::table! {
    nft_collection_bids (bid_obj_addr) {
        #[max_length = 300]
        bid_obj_addr -> Varchar,
        #[max_length = 300]
        collection_addr -> Varchar,
        nft_standard -> Int4,
        #[max_length = 300]
        marketplace_addr -> Varchar,
        #[max_length = 300]
        buyer_addr -> Varchar,
        price -> Int8,
        #[max_length = 300]
        payment_token -> Varchar,
        payment_token_type -> Int4,
        create_timestamp -> Int8,
        last_update_timestamp -> Int8,
        last_update_event_idx -> Int8,
        order_status -> Int4,
    }
}

diesel::table! {
    package_upgrade_history (package_addr, package_name, upgrade_number) {
        #[max_length = 300]
        package_addr -> Varchar,
        #[max_length = 300]
        package_name -> Varchar,
        upgrade_number -> Int8,
        upgrade_policy -> Int8,
        package_manifest -> Text,
        source_digest -> Text,
        tx_version -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    contract_upgrade_ledger_infos,
    contract_upgrade_processor_status,
    marketplace_ledger_infos,
    marketplace_processor_status,
    module_upgrade_history,
    nft_asks,
    nft_bids,
    nft_collection_bids,
    package_upgrade_history,
);
