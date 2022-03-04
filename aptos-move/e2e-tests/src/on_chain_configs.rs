// Copyright (c) The Aptos Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{account::Account, executor::FakeExecutor};
use diem_framework_releases::legacy::transaction_scripts::LegacyStdlibScript;
use aptos_types::{
    on_chain_config::AptosVersion,
    transaction::{Script, TransactionArgument},
};
use aptos_vm::AptosVM;

pub fn set_aptos_version(executor: &mut FakeExecutor, version: AptosVersion) {
    let account = Account::new_genesis_account(aptos_types::on_chain_config::config_address());
    let txn = account
        .transaction()
        .script(Script::new(
            LegacyStdlibScript::UpdateDiemVersion
                .compiled_bytes()
                .into_vec(),
            vec![],
            vec![
                TransactionArgument::U64(0),
                TransactionArgument::U64(version.major),
            ],
        ))
        .sequence_number(0)
        .sign();
    executor.new_block();
    executor.execute_and_apply(txn);

    let new_vm = AptosVM::new(executor.get_state_view());
    assert_eq!(new_vm.internals().aptos_version().unwrap(), version);
}
