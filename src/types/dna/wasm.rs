//! holochain_types::dna::wasm is a module for managing webassembly code
//!  - within the in-memory dna struct
//!  - and serialized to json
use holo_hash::*;
use holochain_serialized_bytes::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};
use wasm_bindgen::prelude::*;

/// Represents web assembly code.
#[derive(Serialize, Deserialize, Clone, Eq)]
pub struct DnaWasm {
    /// the wasm bytes from a .wasm file
    pub code: Arc<Vec<u8>>,
}

/// A DnaWasm paired with its WasmHash
pub type DnaWasmHashed = HoloHashed<DnaWasm>;

impl HashableContent for DnaWasm {
    type HashType = hash_type::Wasm;

    fn hash_type(&self) -> Self::HashType {
        hash_type::Wasm
    }

    fn hashable_content(&self) -> HashableContentBytes {
        HashableContentBytes::Content(
            self.try_into()
                .expect("Could not serialize HashableContent"),
        )
    }
}

impl TryFrom<&DnaWasm> for SerializedBytes {
    type Error = SerializedBytesError;
    fn try_from(dna_wasm: &DnaWasm) -> Result<Self, Self::Error> {
        Ok(SerializedBytes::from(UnsafeBytes::from(
            (*dna_wasm.code).to_owned(),
        )))
    }
}
impl TryFrom<DnaWasm> for SerializedBytes {
    type Error = SerializedBytesError;
    fn try_from(dna_wasm: DnaWasm) -> Result<Self, Self::Error> {
        Self::try_from(&dna_wasm)
    }
}

impl TryFrom<SerializedBytes> for DnaWasm {
    type Error = SerializedBytesError;
    fn try_from(serialized_bytes: SerializedBytes) -> Result<Self, Self::Error> {
        Ok(DnaWasm {
            code: Arc::new(serialized_bytes.bytes().to_vec()),
        })
    }
}

impl DnaWasm {
    /// Provide basic placeholder for wasm entries in dna structs, used for testing only.
    pub fn new_invalid() -> Self {
        DnaWasm {
            code: Arc::new(vec![]),
        }
    }
}

impl fmt::Debug for DnaWasm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<<<DNA WASM CODE>>>")
    }
}

impl PartialEq for DnaWasm {
    fn eq(&self, other: &DnaWasm) -> bool {
        self.code == other.code
    }
}

impl Hash for DnaWasm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl From<Vec<u8>> for DnaWasm {
    fn from(wasm: Vec<u8>) -> Self {
        Self {
            code: Arc::new(wasm),
        }
    }
}
