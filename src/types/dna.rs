//! dna is a library for working with holochain dna files/entries.
//!
//! It includes utilities for representing dna structures in memory,
//! as well as serializing and deserializing dna, mainly to json format.

pub mod error;
pub mod wasm;
pub mod zome;
use self::{
    error::DnaResult,
    zome::{Zome, ZomeDef},
};
use super::prelude::*;
pub use error::DnaError;
use holo_hash::impl_hashable_content;
pub use holo_hash::*;
use holochain_zome_types::zome::ZomeName;
use std::collections::BTreeMap;

/// Zomes need to be an ordered map from ZomeName to a Zome
pub type Zomes = Vec<(ZomeName, zome::ZomeDef)>;

/// A type to allow json values to be used as [SerializedBytes]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, SerializedBytes)]
pub struct JsonProperties(serde_json::Value);

impl JsonProperties {
    /// Create new properties from json value
    pub fn new(properties: serde_json::Value) -> Self {
        JsonProperties(properties)
    }
}

/// Represents the top-level holochain dna object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, SerializedBytes)]
pub struct DnaDef {
    /// The friendly "name" of a Holochain DNA.
    pub name: String,

    /// A UUID for uniquifying this Dna.
    // TODO: consider Vec<u8> instead (https://github.com/holochain/holochain/pull/86#discussion_r412689085)
    pub uuid: String,

    /// Any arbitrary application properties can be included in this object.
    pub properties: SerializedBytes,

    /// An array of zomes associated with your holochain application.
    pub zomes: Zomes,
}

impl DnaDef {
    /// Return a Zome
    pub fn get_zome(&self, zome_name: &ZomeName) -> Result<zome::Zome, DnaError> {
        self.zomes
            .iter()
            .find(|(name, _)| name == zome_name)
            .cloned()
            .map(|(name, def)| Zome::new(name, def))
            .ok_or_else(|| DnaError::ZomeNotFound(format!("Zome '{}' not found", &zome_name,)))
    }

    /// Return a Zome, error if not a WasmZome
    pub fn get_wasm_zome(&self, zome_name: &ZomeName) -> Result<&zome::ZomeDef, DnaError> {
        self.zomes
            .iter()
            .find(|(name, _)| name == zome_name)
            .map(|(_, def)| def)
            .ok_or_else(|| DnaError::ZomeNotFound(format!("Zome '{}' not found", &zome_name,)))
            .and_then(|def| {
                if let ZomeDef{wasm_hash}= def {
                    Ok(def)
                } else {
                    Err(DnaError::NonWasmZome(zome_name.clone()))
                }
            })
    }
}

/// A DnaDef paired with its DnaHash
pub type DnaDefHashed = HoloHashed<DnaDef>;

impl_hashable_content!(DnaDef, Dna);

/// Wasms need to be an ordered map from WasmHash to a wasm::DnaWasm
pub type Wasms = BTreeMap<holo_hash::WasmHash, wasm::DnaWasm>;

/// Represents a full DNA file including WebAssembly bytecode.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, SerializedBytes)]
pub struct DnaFile {
    /// The hashable portion that can be shared with hApp code.
    pub dna: DnaDefHashed,

    /// The bytes of the WASM zomes referenced in the Dna portion.
    pub code: Wasms,
}

impl From<DnaFile> for (DnaDef, Vec<wasm::DnaWasm>) {
    fn from(dna_file: DnaFile) -> (DnaDef, Vec<wasm::DnaWasm>) {
        (
            dna_file.dna.into_content(),
            dna_file.code.into_iter().map(|(_, w)| w).collect(),
        )
    }
}

impl DnaFile {
    /// Construct a new DnaFile instance.
    pub async fn new(
        dna: DnaDef,
        wasm: impl IntoIterator<Item = wasm::DnaWasm>,
    ) -> Result<Self, DnaError> {
        let mut code = BTreeMap::new();
        for wasm in wasm {
            let wasm_hash = holo_hash::WasmHash::with_data(&wasm).await;
            code.insert(wasm_hash, wasm);
        }
        let dna = DnaDefHashed::from_content(dna).await;
        Ok(Self { dna, code })
    }

    /// The DnaDef along with its hash
    pub fn dna(&self) -> &DnaDefHashed {
        &self.dna
    }

    /// Just the DnaDef
    pub fn dna_def(&self) -> &DnaDef {
        &self.dna
    }

    /// The hash of the DnaDef
    pub fn dna_hash(&self) -> &holo_hash::DnaHash {
        self.dna.as_hash()
    }

    /// Verify that the DNA hash in the file matches the DnaDef
    pub async fn verify_hash(&self) -> Result<(), DnaError> {
        self.dna
            .verify_hash()
            .await
            .map_err(|hash| DnaError::DnaHashMismatch(self.dna.as_hash().clone(), hash))
    }

    /// Transform this DnaFile into a new DnaFile with different properties
    /// and, hence, a different DnaHash.
    pub async fn with_properties(self, properties: SerializedBytes) -> Result<Self, DnaError> {
        let (mut dna, wasm): (DnaDef, Vec<wasm::DnaWasm>) = self.into();
        dna.properties = properties;
        DnaFile::new(dna, wasm).await
    }

    /// Transform this DnaFile into a new DnaFile with a different UUID
    /// and, hence, a different DnaHash.
    pub async fn with_uuid(self, uuid: String) -> Result<Self, DnaError> {
        let (mut dna, wasm): (DnaDef, Vec<wasm::DnaWasm>) = self.into();
        dna.uuid = uuid;
        DnaFile::new(dna, wasm).await
    }

    /// The bytes of the WASM zomes referenced in the Dna portion.
    pub fn code(&self) -> &BTreeMap<holo_hash::WasmHash, wasm::DnaWasm> {
        &self.code
    }


    /// Render this dna_file as bytecode to send over the wire, or store in a file.
    pub async fn to_file_content(&self) -> Result<Vec<u8>, DnaError> {
        // Not super efficient memory-wise, but doesn't block any threads
        let dna_file = self.clone();
        // TODO: remove
        dna_file.verify_hash().await.expect("TODO, remove");
        let data: SerializedBytes = dna_file.try_into()?;

        let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        use std::io::Write;
        enc.write_all(data.bytes())?;
        Ok(enc.finish()?)
    }
}

impl std::fmt::Debug for DnaFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("DnaFile(dna_hash = {})", self.dna_hash()))
    }
}
