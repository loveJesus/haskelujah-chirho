// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Driver-owned, embedded actual-source slices. No developer reference path is consulted.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::{NextErrorChirho, NextErrorKindChirho};
use haskelujah_hir_chirho::*;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
mod boundaries_chirho;
mod prelude_scope_chirho;
pub(super) use prelude_scope_chirho::exports_chirho as prelude_exports_chirho;

pub(super) const NUM_MODULE_CHIRHO: &str = "GHC.Internal.Num";
pub(super) const INTEGER_MODULE_CHIRHO: &str = "GHC.Internal.Bignum.Integer";
pub(super) const LICENSE_CHIRHO: &str = include_str!("vendor-chirho/ghc-internal-chirho/LICENSE");

pub(super) struct SourceManifestChirho {
    pub ghc_version_chirho: &'static str,
    pub module_chirho: &'static str,
    pub upstream_path_chirho: &'static str,
    pub sha256_chirho: &'static str,
    pub bytes_chirho: &'static str,
    pub spans_chirho: &'static [(usize, usize)],
    pub imports_chirho: &'static str,
}
const TUPLE_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    ghc_version_chirho: "9.14.1",
    module_chirho: "GHC.Internal.Data.Tuple",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/Data/Tuple.hs",
    sha256_chirho: "97658f1c5b9be7910c23480641fb8f1aedbdbe96d0e2d68b5c6106aa48e7a7ad",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/data-chirho/Tuple.hs"),
    spans_chirho: &[(926, 993), (1039, 1106)],
    imports_chirho: "",
};
const NUM_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    ghc_version_chirho: "9.14.1",
    module_chirho: NUM_MODULE_CHIRHO,
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/Num.hs",
    sha256_chirho: "a35a0fac5e44dde385bc98024d85a82924dac33c59de0996c9978e4295e4077a",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/Num.hs"),
    spans_chirho: &[(1066, 1093), (2395, 3302)],
    imports_chirho: "import GHC.Internal.Bignum.Integer (Integer)\n",
};

pub(super) fn manifest_chirho(module_chirho: &str) -> Option<&'static SourceManifestChirho> {
    match module_chirho {
        "GHC.Internal.Data.Tuple" => Some(&TUPLE_CHIRHO),
        NUM_MODULE_CHIRHO => Some(&NUM_CHIRHO),
        _ => None,
    }
}
fn verify_source_chirho(
    bytes_chirho: &str,
    expected_chirho: &str,
    path_chirho: &str,
    version_chirho: &str,
) -> Result<(), NextErrorChirho> {
    if Sha256::digest(bytes_chirho.as_bytes())
        .iter()
        .map(|byte_chirho| format!("{byte_chirho:02x}"))
        .collect::<String>()
        != expected_chirho
    {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            format!("GHC {version_chirho} source manifest hash mismatch: {path_chirho}"),
        ));
    }
    Ok(())
}
impl SourceManifestChirho {
    /// Hash the whole unmodified upstream file before selecting declared UTF-8 byte spans.
    pub(super) fn source_chirho(&self) -> Result<String, NextErrorChirho> {
        verify_source_chirho(
            self.bytes_chirho,
            self.sha256_chirho,
            self.upstream_path_chirho,
            self.ghc_version_chirho,
        )?;
        let mut source_chirho = format!(
            "{{-# LANGUAGE Haskell2010, NoImplicitPrelude #-}}\nmodule {} where\n{}",
            self.module_chirho, self.imports_chirho
        );
        let mut previous_chirho = 0;
        for &(start_chirho, end_chirho) in self.spans_chirho {
            if start_chirho < previous_chirho || start_chirho >= end_chirho {
                return Err(NextErrorChirho::new_chirho(
                    NextErrorKindChirho::UnsupportedChirho,
                    "invalid source manifest span ordering",
                ));
            }
            let selection_chirho =
                self.bytes_chirho
                    .get(start_chirho..end_chirho)
                    .ok_or_else(|| {
                        NextErrorChirho::new_chirho(
                            NextErrorKindChirho::UnsupportedChirho,
                            "invalid source manifest byte span",
                        )
                    })?;
            boundaries_chirho::validate_chirho(self.bytes_chirho, start_chirho, end_chirho)?;
            source_chirho.push_str(selection_chirho);
            source_chirho.push('\n');
            previous_chirho = end_chirho;
        }
        Ok(source_chirho)
    }
}

/// Integer is wired in GHC; Num is checked source, never a wired class catalogue.
pub(super) fn integer_interface_chirho() -> InterfaceChirho {
    let original_chirho = OriginalNameChirho {
        unit_chirho: UnitIdentityChirho::CompilerChirho,
        module_chirho: INTEGER_MODULE_CHIRHO.into(),
        namespace_chirho: NamespaceChirho::TypeChirho,
        occurrence_chirho: "Integer".into(),
    };
    InterfaceChirho {
        unit_chirho: UnitIdentityChirho::CompilerChirho,
        module_chirho: INTEGER_MODULE_CHIRHO.into(),
        completeness_chirho: CompletenessChirho::SliceChirho,
        sliced_units_chirho: BTreeSet::from([UnitIdentityChirho::CompilerChirho]),
        exports_chirho: vec![original_chirho.clone()],
        wired_types_chirho: BTreeMap::from([(original_chirho, WiredNameChirho::IntegerChirho)]),
        fixities_chirho: BTreeMap::new(),
        data_chirho: BTreeMap::new(),
        values_chirho: BTreeMap::new(),
        classes_chirho: BTreeMap::new(),
        instances_chirho: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests_chirho;
