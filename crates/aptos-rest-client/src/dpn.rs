// Copyright (c) The Aptos Core Contributors
// SPDX-License-Identifier: Apache-2.0

use aptos_api_types::U64;
use move_core_types::{identifier::Identifier, language_storage::StructTag};
use serde::{Deserialize, Serialize};

pub use aptos_types::account_config::{aptos_root_address, BalanceResource, CORE_CODE_ADDRESS};
pub use aptos_types::on_chain_config::{
    config_struct_tag, ConfigurationResource, AptosVersion as OnChainAptosVersion, OnChainConfig,
};

use crate::types::EventHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Aptos {
    pub value: U64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub coin: Aptos,
}

#[derive(Debug)]
pub struct AccountBalance {
    pub currency: StructTag,
    pub amount: u64,
}

impl AccountBalance {
    pub fn currency_code(&self) -> String {
        self.currency.name.to_string()
    }
}

pub fn aptos_version_identifier() -> Identifier {
    Identifier::new(OnChainAptosVersion::IDENTIFIER).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AptosConfig<T> {
    pub payload: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AptosVersion {
    pub major: U64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "epoch")]
    pub next_block_epoch: U64,
    pub last_reconfiguration_time: U64,
    pub events: EventHandle,
}
