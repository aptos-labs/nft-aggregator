use ahash::{AHashMap, AHashSet};
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::{
        write_set_change::Change, Event as EventPB, MoveModuleBytecode, Transaction, WriteSetChange,
    },
    traits::{async_step::AsyncRunType, AsyncStep, NamedStep, Processable},
    types::transaction_context::TransactionContext,
    utils::{convert::standardize_address, errors::ProcessorError},
};
use async_trait::async_trait;
use rayon::prelude::*;

use crate::db_models::{
    module_upgrade::ModuleUpgrade,
    package_upgrade::{PackageUpgrade, PackageUpgradeChangeOnChain},
};

/// Extractor is a step that extracts events and their metadata from transactions.
pub struct Extractor
where
    Self: Sized + Send + 'static,
{
    contract_addresses: AHashSet<String>,
}

impl Extractor {
    pub fn new(contract_addresses: Vec<String>) -> Self {
        Self {
            contract_addresses: contract_addresses.into_iter().collect(),
        }
    }
}

impl AsyncStep for Extractor {}

impl NamedStep for Extractor {
    fn name(&self) -> String {
        "Extractor".to_string()
    }
}

#[async_trait]
impl Processable for Extractor {
    type Input = Vec<Transaction>;
    type Output = TransactionContextData;
    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        item: TransactionContext<Vec<Transaction>>,
    ) -> Result<Option<TransactionContext<TransactionContextData>>, ProcessorError> {
        let results: Vec<(Vec<_>, Vec<ContractUpgradeChange>)> = item
            .data
            .par_iter()
            .map(|txn| {
                let txn_version = txn.version as i64;
                let txn_info = match txn.info.as_ref() {
                    Some(info) => info,
                    None => {
                        tracing::warn!(
                            transaction_version = txn_version,
                            "Transaction info doesn't exist"
                        );
                        return (vec![], vec![]);
                    }
                };

                let txn_changes = ContractUpgradeChange::from_changes(
                    &self.contract_addresses,
                    txn_version,
                    txn_info.changes.as_slice(),
                );

                (vec![], txn_changes)
            })
            .collect::<Vec<(Vec<_>, Vec<ContractUpgradeChange>)>>();

        let (events, changes): (Vec<EventPB>, Vec<ContractUpgradeChange>) =
            results.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut events_acc, mut changes_acc), (events, changes)| {
                    events_acc.extend(events);
                    changes_acc.extend(changes);
                    (events_acc, changes_acc)
                },
            );

        Ok(Some(TransactionContext {
            data: TransactionContextData { events, changes },
            metadata: item.metadata,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct TransactionContextData {
    pub events: Vec<EventPB>,
    pub changes: Vec<ContractUpgradeChange>,
}

#[derive(Debug, Clone)]
pub enum ContractUpgradeChange {
    ModuleUpgradeChange(ModuleUpgrade),
    PackageUpgradeChange(PackageUpgrade),
}

impl ContractUpgradeChange {
    pub fn from_changes(
        contract_addresses: &AHashSet<String>,
        txn_version: i64,
        changes: &[WriteSetChange],
    ) -> Vec<Self> {
        // key is (contract_address, module_name), value is MoveModuleBytecode
        let mut raw_module_changes: AHashMap<(String, String), MoveModuleBytecode> =
            AHashMap::new();
        // (package_address, package_upgrade_change)
        let mut raw_package_changes: Vec<(String, PackageUpgradeChangeOnChain)> = vec![];

        changes.iter().for_each(|change| {
            if let Some(change) = change.change.as_ref() {
                match change {
                    Change::WriteModule(write_module_change) => {
                        if contract_addresses.contains(
                            standardize_address(write_module_change.address.as_str()).as_str(),
                        ) {
                            raw_module_changes.insert(
                                (
                                    standardize_address(write_module_change.address.as_str()),
                                    write_module_change
                                        .data
                                        .clone()
                                        .unwrap_or_else(|| {
                                            panic!("MoveModuleBytecode data is missing",)
                                        })
                                        .abi
                                        .clone()
                                        .unwrap_or_else(|| {
                                            panic!("MoveModuleBytecode abi is missing",)
                                        })
                                        .name,
                                ),
                                write_module_change.data.clone().unwrap(),
                            );
                        }
                    }
                    Change::WriteResource(write_resource_change) => {
                        if contract_addresses.contains(
                            standardize_address(write_resource_change.address.as_str()).as_str(),
                        ) && write_resource_change.type_str == "0x1::code::PackageRegistry"
                        {
                            let package_upgrade: PackageUpgradeChangeOnChain =
                                serde_json::from_str(write_resource_change.data.as_str())
                                    .unwrap_or_else(|_| {
                                        panic!(
                                            "Failed to parse PackageUpgradeChangeOnChain, {}",
                                            write_resource_change.data.as_str()
                                        )
                                    });
                            raw_package_changes.push((
                                standardize_address(write_resource_change.address.as_str()),
                                package_upgrade,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        });

        let package_changes = raw_package_changes
            .iter()
            .flat_map(|(package_address, package_change)| {
                package_change.to_db_package_upgrade(txn_version, package_address.clone())
            })
            .collect::<Vec<PackageUpgrade>>();

        let module_changes = raw_package_changes
            .iter()
            .flat_map(|(package_address, package_change)| {
                package_change.packages.iter().flat_map(|package| {
                    package
                        .modules
                        .iter()
                        .filter_map(|module| {
                            // If raw module is missing, it means the module is not changed
                            // This happens when developer published a new package at the same address
                            // All the modules from the previous package are unchanged but still in the write set change
                            let raw_module = raw_module_changes
                                .get(&(package_address.clone(), module.name.clone()));
                            raw_module.map(|raw_module| ModuleUpgrade {
                                module_addr: package_address.clone(),
                                module_name: module.name.clone(),
                                package_name: package.name.clone(),
                                upgrade_number: package.upgrade_number.parse().unwrap(),
                                module_bytecode: raw_module.bytecode.clone(),
                                module_source_code: module.source.clone(),
                                module_abi: serde_json::json!(raw_module
                                    .abi
                                    .clone()
                                    .unwrap_or_else(|| {
                                        panic!("Module abi is missing for module {}", module.name)
                                    })),
                                tx_version: txn_version,
                            })
                        })
                        .collect::<Vec<ModuleUpgrade>>()
                })
            })
            .collect::<Vec<ModuleUpgrade>>();

        module_changes
            .into_iter()
            .map(ContractUpgradeChange::ModuleUpgradeChange)
            .chain(
                package_changes
                    .into_iter()
                    .map(ContractUpgradeChange::PackageUpgradeChange),
            )
            .collect()
    }
}
