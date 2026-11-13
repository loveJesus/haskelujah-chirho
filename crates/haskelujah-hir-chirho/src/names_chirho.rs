// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

pub use haskelujah_naming_chirho::env_chirho::NamespaceChirho;
use haskelujah_span_chirho::SpanChirho;

/// A module-local index; the owning module retains its defining unit and module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameIdChirho(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitIdentityChirho {
    /// Minted by the standalone driver mode, never inferred from a path.
    StandaloneHomeChirho,
    /// GHC 9.14.1 fixed ghc-internal unit; source admission needs a driver manifest.
    CompilerChirho,
}

/// Only wired-in language entities, not a replacement Prelude catalogue.
/// Reference: GHC.Builtin.Types, boolTyCon/trueDataCon/falseDataCon/charTyCon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WiredNameChirho {
    BoolChirho,
    CharChirho,
    TrueChirho,
    FalseChirho,
}

#[derive(Debug, Clone)]
pub struct NameInfoChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub namespace_chirho: NamespaceChirho,
    pub module_chirho: String,
    pub occurrence_chirho: String,
    pub local_chirho: bool,
    pub wired_chirho: Option<WiredNameChirho>,
    pub span_chirho: SpanChirho,
}

/// A declaration's defining identity. Import paths and arena indices are not identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OriginalNameChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub module_chirho: String,
    pub namespace_chirho: NamespaceChirho,
    pub occurrence_chirho: String,
}

impl NameInfoChirho {
    pub fn original_chirho(&self) -> OriginalNameChirho {
        OriginalNameChirho {
            unit_chirho: self.unit_chirho.clone(),
            module_chirho: self.module_chirho.clone(),
            namespace_chirho: self.namespace_chirho,
            occurrence_chirho: self.occurrence_chirho.clone(),
        }
    }
}

impl OriginalNameChirho {
    /// Length-prefix every freely chosen component; separators cannot collide.
    pub fn link_key_chirho(&self, role_chirho: &str) -> String {
        let unit_chirho = match self.unit_chirho {
            UnitIdentityChirho::StandaloneHomeChirho => "main",
            UnitIdentityChirho::CompilerChirho => "ghc-internal",
        };
        let namespace_chirho = match self.namespace_chirho {
            NamespaceChirho::TypeChirho => "type",
            NamespaceChirho::ValueChirho => "value",
        };
        let mut result_chirho = String::from("%next:original:");
        for part_chirho in [
            unit_chirho,
            self.module_chirho.as_str(),
            namespace_chirho,
            self.occurrence_chirho.as_str(),
            role_chirho,
        ] {
            result_chirho.push_str(&format!("{}#{part_chirho}", part_chirho.len()));
        }
        result_chirho
    }
}
