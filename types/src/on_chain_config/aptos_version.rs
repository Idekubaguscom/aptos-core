// Copyright (c) The Aptos Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use serde::{Deserialize, Serialize};

/// Defines the version of Aptos Validator software.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AptosVersion {
    pub major: u64,
}

impl OnChainConfig for AptosVersion {
    const IDENTIFIER: &'static str = "AptosVersion";
}

// NOTE: version number for release 1.2 Aptos
// Items gated by this version number include:
//  - the ScriptFunction payload type
pub const DIEM_VERSION_2: AptosVersion = AptosVersion { major: 2 };

// NOTE: version number for release 1.3 of Aptos
// Items gated by this version number include:
//  - Multi-agent transactions
pub const DIEM_VERSION_3: AptosVersion = AptosVersion { major: 3 };

// NOTE: version number for release 1.4 of Aptos
// Items gated by this version number include:
//  - Conflict-Resistant Sequence Numbers
pub const DIEM_VERSION_4: AptosVersion = AptosVersion { major: 4 };

// Maximum current known version
pub const DIEM_MAX_KNOWN_VERSION: AptosVersion = DIEM_VERSION_4;
