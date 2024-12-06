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
    module_upgrade_history,
    package_upgrade_history,
);
