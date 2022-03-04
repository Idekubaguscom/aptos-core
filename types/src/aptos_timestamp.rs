// Copyright (c) The Aptos Core Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AptosTimestampResource {
    pub aptos_timestamp: AptosTimestamp,
}

impl MoveStructType for AptosTimestampResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("AptosTimestamp");
    const STRUCT_NAME: &'static IdentStr = ident_str!("CurrentTimeMicroseconds");
}

impl MoveResource for AptosTimestampResource {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AptosTimestamp {
    pub microseconds: u64,
}
