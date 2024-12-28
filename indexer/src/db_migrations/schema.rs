// @generated automatically by Diesel CLI.

diesel::table! {
    collection_bids (bid_obj_addr) {
        #[max_length = 300]
        bid_obj_addr -> Varchar,
        #[max_length = 300]
        collection_addr -> Nullable<Varchar>,
        #[max_length = 300]
        collection_creator_addr -> Varchar,
        #[max_length = 300]
        collection_name -> Varchar,
        nft_standard -> Int4,
        #[max_length = 300]
        marketplace_addr -> Varchar,
        #[max_length = 300]
        buyer_addr -> Varchar,
        total_nft_amount -> Int8,
        remaining_nft_amount -> Int8,
        price -> Int8,
        #[max_length = 300]
        payment_token -> Varchar,
        payment_token_type -> Int4,
        order_placed_timestamp -> Int8,
        order_placed_tx_version -> Int8,
        order_placed_event_idx -> Int8,
        latest_order_filled_timestamp -> Int8,
        latest_order_filled_tx_version -> Int8,
        latest_order_filled_event_idx -> Int8,
        order_cancelled_timestamp -> Int8,
        order_cancelled_tx_version -> Int8,
        order_cancelled_event_idx -> Int8,
        order_status -> Int4,
    }
}

diesel::table! {
    filled_collection_bids (bid_obj_addr, nft_id, nft_name) {
        #[max_length = 300]
        bid_obj_addr -> Varchar,
        #[max_length = 300]
        nft_id -> Varchar,
        #[max_length = 300]
        nft_name -> Varchar,
        price -> Int8,
        royalties -> Int8,
        commission -> Int8,
        order_filled_timestamp -> Int8,
        order_filled_tx_version -> Int8,
        order_filled_event_idx -> Int8,
    }
}

diesel::table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
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
    nft_asks (ask_obj_addr) {
        #[max_length = 300]
        ask_obj_addr -> Varchar,
        #[max_length = 300]
        nft_id -> Varchar,
        #[max_length = 300]
        nft_name -> Varchar,
        #[max_length = 300]
        collection_addr -> Nullable<Varchar>,
        #[max_length = 300]
        collection_creator_addr -> Varchar,
        #[max_length = 300]
        collection_name -> Varchar,
        nft_standard -> Int4,
        #[max_length = 300]
        marketplace_addr -> Varchar,
        #[max_length = 300]
        seller_addr -> Varchar,
        price -> Int8,
        royalties -> Int8,
        commission -> Int8,
        #[max_length = 300]
        payment_token -> Varchar,
        payment_token_type -> Int4,
        order_placed_timestamp -> Int8,
        order_placed_tx_version -> Int8,
        order_placed_event_idx -> Int8,
        order_filled_timestamp -> Int8,
        order_filled_tx_version -> Int8,
        order_filled_event_idx -> Int8,
        order_cancelled_timestamp -> Int8,
        order_cancelled_tx_version -> Int8,
        order_cancelled_event_idx -> Int8,
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
        nft_name -> Varchar,
        #[max_length = 300]
        collection_addr -> Nullable<Varchar>,
        #[max_length = 300]
        collection_creator_addr -> Varchar,
        #[max_length = 300]
        collection_name -> Varchar,
        nft_standard -> Int4,
        #[max_length = 300]
        marketplace_addr -> Varchar,
        #[max_length = 300]
        buyer_addr -> Varchar,
        price -> Int8,
        royalties -> Int8,
        commission -> Int8,
        #[max_length = 300]
        payment_token -> Varchar,
        payment_token_type -> Int4,
        order_placed_timestamp -> Int8,
        order_placed_tx_version -> Int8,
        order_placed_event_idx -> Int8,
        order_filled_timestamp -> Int8,
        order_filled_tx_version -> Int8,
        order_filled_event_idx -> Int8,
        order_cancelled_timestamp -> Int8,
        order_cancelled_tx_version -> Int8,
        order_cancelled_event_idx -> Int8,
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

diesel::table! {
    processor_status (processor) {
        #[max_length = 50]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::joinable!(filled_collection_bids -> collection_bids (bid_obj_addr));

diesel::allow_tables_to_appear_in_same_query!(
    collection_bids,
    filled_collection_bids,
    ledger_infos,
    module_upgrade_history,
    nft_asks,
    nft_bids,
    package_upgrade_history,
    processor_status,
);
