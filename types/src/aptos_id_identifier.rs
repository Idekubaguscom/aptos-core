// Copyright (c) The Aptos Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This file implements length and character set limited string types needed
//! for AptosID as defined by DIP-10: https://github.com/aptos/dip/blob/main/dips/dip-10.md

use serde::{de, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AptosId {
    // A reusable identifier that represents either the source or destination
    // end-user of a payment transaction. It is unique to per user at VASP level
    user_identifier: AptosIdUserIdentifier,
    // A unique string that is mapped to a VASP
    vasp_domain_identifier: AptosIdVaspDomainIdentifier,
}

impl AptosId {
    pub fn new(
        user_identifier: AptosIdUserIdentifier,
        vasp_domain_identifier: AptosIdVaspDomainIdentifier,
    ) -> Self {
        Self {
            user_identifier,
            vasp_domain_identifier,
        }
    }

    fn new_from_raw(
        user_identifier: &str,
        vasp_domain_identifier: &str,
    ) -> Result<Self, AptosIdParseError> {
        Ok(AptosId::new(
            AptosIdUserIdentifier::new(user_identifier)?,
            AptosIdVaspDomainIdentifier::new(vasp_domain_identifier)?,
        ))
    }

    pub fn user_identifier(&self) -> &AptosIdUserIdentifier {
        &self.user_identifier
    }

    pub fn vasp_domain_identifier(&self) -> &AptosIdVaspDomainIdentifier {
        &self.vasp_domain_identifier
    }
}

impl Display for AptosId {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}@{}",
            self.user_identifier.as_str(),
            self.vasp_domain_identifier.as_str()
        )
    }
}

impl FromStr for AptosId {
    type Err = AptosIdParseError;

    fn from_str(s: &str) -> Result<Self, AptosIdParseError> {
        let index = s
            .find('@')
            .ok_or_else(|| AptosIdParseError::new("AptosId does not have @".to_string()))?;
        let split = s.split_at(index);
        AptosId::new_from_raw(split.0, &(split.1)[1..])
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AptosIdUserIdentifier(Box<str>);

impl AptosIdUserIdentifier {
    pub fn new(s: &str) -> Result<Self, AptosIdParseError> {
        if s.len() > 64 || s.is_empty() {
            return Err(AptosIdParseError::new(
                "Invalid length for user identifier".to_string(),
            ));
        }

        let mut chars = s.chars();
        match chars.next() {
            Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') => {}
            Some(_) => {
                return Err(AptosIdParseError::new(
                    "Invalid user identifier input".to_string(),
                ))
            }
            None => {
                return Err(AptosIdParseError::new(
                    "Invalid user identifier input".to_string(),
                ))
            }
        }
        for c in chars {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '.' => {}
                _ => return Err(AptosIdParseError::new(format!("Invalid character {}", c))),
            }
        }
        Ok(AptosIdUserIdentifier(s.into()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> de::Deserialize<'de> for AptosIdUserIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = <String>::deserialize(deserializer)?;
        AptosIdUserIdentifier::new(&s).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AptosIdVaspDomainIdentifier(Box<str>);

impl AptosIdVaspDomainIdentifier {
    pub fn new(s: &str) -> Result<Self, AptosIdParseError> {
        if s.len() > 63 || s.is_empty() {
            return Err(AptosIdParseError::new(
                "Invalid length for vasp domain identifier".to_string(),
            ));
        }

        let mut chars = s.chars();
        match chars.next() {
            Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') => {}
            Some(_) => {
                return Err(AptosIdParseError::new(
                    "Invalid vasp domain input".to_string(),
                ))
            }
            None => {
                return Err(AptosIdParseError::new(
                    "Invalid vasp domain input".to_string(),
                ))
            }
        }
        for c in chars {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '.' => {}
                _ => return Err(AptosIdParseError::new(format!("Invalid character {}", c))),
            }
        }
        Ok(AptosIdVaspDomainIdentifier(s.into()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> de::Deserialize<'de> for AptosIdVaspDomainIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = <String>::deserialize(deserializer)?;
        AptosIdVaspDomainIdentifier::new(&s).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AptosIdParseError {
    message: String,
}

impl AptosIdParseError {
    fn new(message: String) -> AptosIdParseError {
        AptosIdParseError { message }
    }
}

impl Display for AptosIdParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Unable to parse AptosId: {}", self.message)
    }
}

impl std::error::Error for AptosIdParseError {}

#[test]
fn test_invalid_user_identifier() {
    // Test valid domain
    let raw_identifier = "abcd1234";
    let identifier = AptosIdUserIdentifier::new(raw_identifier);
    assert!(identifier.is_ok());

    // Test having 64 characters is valid
    let raw_identifier = "aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffff1234";
    let identifier = AptosIdUserIdentifier::new(raw_identifier);
    assert!(identifier.is_ok());

    // Test using "-" character is invalid
    let raw_identifier = "abcd!!!1234";
    let identifier = AptosIdUserIdentifier::new(raw_identifier);
    assert_eq!(
        identifier.unwrap_err(),
        AptosIdParseError {
            message: "Invalid character !".to_string()
        },
    );
    // Test having 64 characters is invalid
    let raw_identifier = "aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffff12345";
    let identifier = AptosIdUserIdentifier::new(raw_identifier);
    assert_eq!(
        identifier.unwrap_err(),
        AptosIdParseError {
            message: "Invalid length for user identifier".to_string()
        },
    );
}

#[test]
fn test_invalid_vasp_domain_identifier() {
    // Test valid domain
    let raw_identifier = "aptos";
    let identifier = AptosIdVaspDomainIdentifier::new(raw_identifier);
    assert!(identifier.is_ok());

    // Test having 63 characters is valid
    let raw_identifier = "aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffff123";
    let identifier = AptosIdVaspDomainIdentifier::new(raw_identifier);
    assert!(identifier.is_ok());

    // Test using "-" character is invalid
    let raw_identifier = "aptos-domain";
    let identifier = AptosIdVaspDomainIdentifier::new(raw_identifier);
    assert_eq!(
        identifier.unwrap_err(),
        AptosIdParseError {
            message: "Invalid character -".to_string()
        },
    );
    // Test having 64 characters is invalid
    let raw_identifier = "aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggg";
    let identifier = AptosIdVaspDomainIdentifier::new(raw_identifier);
    assert_eq!(
        identifier.unwrap_err(),
        AptosIdParseError {
            message: "Invalid length for vasp domain identifier".to_string()
        },
    );
}

#[test]
fn test_get_aptos_id_string_from_components() {
    let user_identifier = AptosIdUserIdentifier::new("username").unwrap();
    let vasp_domain_identifier = AptosIdVaspDomainIdentifier::new("aptos").unwrap();
    let aptos_id = AptosId::new(user_identifier, vasp_domain_identifier);
    let full_id = aptos_id.to_string();
    assert_eq!(full_id, "username@aptos");
}

#[test]
fn test_get_aptos_id_from_identifier_string() {
    let aptos_id_str = "username@aptos";
    let aptos_id = AptosId::from_str(aptos_id_str).unwrap();
    assert_eq!(aptos_id.user_identifier().as_str(), "username");
    assert_eq!(aptos_id.vasp_domain_identifier().as_str(), "aptos");

    let aptos_id_str = "username@aptos.com";
    let aptos_id = AptosId::from_str(aptos_id_str).unwrap();
    assert_eq!(aptos_id.user_identifier().as_str(), "username");
    assert_eq!(aptos_id.vasp_domain_identifier().as_str(), "aptos.com");

    let aptos_id_str = "username@aptos@com";
    let aptos_id = AptosId::from_str(aptos_id_str);
    assert_eq!(
        aptos_id.unwrap_err(),
        AptosIdParseError {
            message: "Invalid character @".to_string()
        },
    );

    let aptos_id_str = "username@@aptos.com";
    let aptos_id = AptosId::from_str(aptos_id_str);
    assert_eq!(
        aptos_id.unwrap_err(),
        AptosIdParseError {
            message: "Invalid vasp domain input".to_string()
        },
    );
}
