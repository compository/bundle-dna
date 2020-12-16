//! A `Zome` is a module of app-defined code which can be run by Holochain.
//! A group of Zomes are composed to form a `Dna`.
//!
//! Real-world Holochain Zomes are written in Wasm.
//! This module also provides for an "inline" zome definition, which is written
//! using Rust closures, and is useful for quickly defining zomes on-the-fly
//! for tests.

use holo_hash::WasmHash;
use holochain_serialized_bytes::prelude::*;
use holochain_zome_types::zome::ZomeName;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use super::{error::DnaResult, DnaError};

/// A Holochain Zome. Includes the ZomeDef as well as the name of the Zome.
#[derive(Serialize, Deserialize, Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zome {
    name: ZomeName,
    def: ZomeDef,
}

impl Zome {
    /// Constructor
    pub fn new(name: ZomeName, def: ZomeDef) -> Self {
        Self { name, def }
    }

    /// Accessor
    pub fn zome_name(&self) -> &ZomeName {
        &self.name
    }

    /// Accessor
    pub fn zome_def(&self) -> &ZomeDef {
        &self.def
    }

    /// Split into components
    pub fn into_inner(self) -> (ZomeName, ZomeDef) {
        (self.name, self.def)
    }
}

impl From<(ZomeName, ZomeDef)> for Zome {
    fn from(pair: (ZomeName, ZomeDef)) -> Self {
        Self::new(pair.0, pair.1)
    }
}

impl From<Zome> for (ZomeName, ZomeDef) {
    fn from(zome: Zome) -> Self {
        zome.into_inner()
    }
}

impl From<Zome> for ZomeName {
    fn from(zome: Zome) -> Self {
        zome.name
    }
}

impl From<Zome> for ZomeDef {
    fn from(zome: Zome) -> Self {
        zome.def
    }
}

/// Just the definition of a Zome, without the name included. This exists
/// mainly for use in HashMaps where ZomeDefs are keyed by ZomeName.
///
/// NB: Only Wasm Zomes are valid to pass through round-trip serialization,
/// because Rust functions are not serializable. Hence, this enum serializes
/// as if it were a bare WasmZome, and when deserializing, only Wasm zomes
/// can be produced. InlineZomes are serialized as their UUID, so that a
/// hash can be computed, but it is invalid to attempt to deserialize them
/// again.
///
/// In particular, a real-world DnaFile should only ever contain Wasm zomes!
#[derive(Serialize, Deserialize, Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ZomeDef {
    pub wasm_hash: WasmHash,
}

/// Access a call has to host functions
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HostFnAccess {
    /// Can access agent information
    pub agent_info: Permission,
    /// Can access the workspace
    pub read_workspace: Permission,
    /// Can write and workspace
    pub write_workspace: Permission,
    /// Can write to the network
    pub write_network: Permission,
    /// Can access dna and zome specific data
    pub dna_bindings: Permission,
    /// All other non-deterministic functions
    pub non_determinism: Permission,
    /// Access to functions that use the keystore in the conductor
    pub keystore: Permission,
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Permission granted to a call
pub enum Permission {
    /// Host functions with this access will be included
    Allow,
    /// Host functions with this access will be unreachable
    Deny,
}
/*
impl ZomeDef {
    /// create a Zome from a holo_hash WasmHash instead of a holo_hash one
    pub fn from_hash(wasm_hash: holo_hash::WasmHash) -> Self {
        WasmZome { wasm_hash }.into()
    }
} */

impl HostFnAccess {
    /// Allow all access
    pub fn all() -> Self {
        HostFnAccess {
            read_workspace: Permission::Allow,
            write_workspace: Permission::Allow,
            agent_info: Permission::Allow,
            non_determinism: Permission::Allow,
            write_network: Permission::Allow,
            keystore: Permission::Allow,
            dna_bindings: Permission::Allow,
        }
    }

    /// Deny all access
    pub fn none() -> Self {
        HostFnAccess {
            read_workspace: Permission::Deny,
            write_workspace: Permission::Deny,
            agent_info: Permission::Deny,
            non_determinism: Permission::Deny,
            write_network: Permission::Deny,
            keystore: Permission::Deny,
            dna_bindings: Permission::Deny,
        }
    }
}
