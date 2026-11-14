// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Driver-owned, embedded actual-source slices. No developer reference path is consulted.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::{NextErrorChirho, NextErrorKindChirho};
use haskelujah_hir_chirho::*;
use sha2::{Digest, Sha256};
mod boundaries_chirho;
mod integer_backend_chirho;
pub(super) use integer_backend_chirho::{
    install_policy_chirho as install_integer_policy_chirho,
    operation_chirho as integer_operation_chirho,
};
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
    pub primitive_signatures_chirho: bool,
}
const TUPLE_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    ghc_version_chirho: "9.14.1",
    module_chirho: "GHC.Internal.Data.Tuple",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/Data/Tuple.hs",
    sha256_chirho: "97658f1c5b9be7910c23480641fb8f1aedbdbe96d0e2d68b5c6106aa48e7a7ad",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/data-chirho/Tuple.hs"),
    spans_chirho: &[(926, 993), (1039, 1106)],
    imports_chirho: "",
    primitive_signatures_chirho: false,
};
const NUM_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    ghc_version_chirho: "9.14.1",
    module_chirho: NUM_MODULE_CHIRHO,
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/Num.hs",
    sha256_chirho: "a35a0fac5e44dde385bc98024d85a82924dac33c59de0996c9978e4295e4077a",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/Num.hs"),
    spans_chirho: &[(1066, 1093), (2395, 3302), (4437, 4637)],
    imports_chirho: "import GHC.Internal.Bignum.Integer (Integer, integerAdd, integerSub, integerMul, integerNegate, integerAbs, integerSignum)\n",
    primitive_signatures_chirho: false,
};
const INTEGER_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    ghc_version_chirho: "9.14.1",
    module_chirho: INTEGER_MODULE_CHIRHO,
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/Bignum/Integer.hs",
    sha256_chirho: "1f8ec2a8e12ecbab7eb0b59f249fae663d8066f2fb14224177a4538653bb17f4",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/bignum-chirho/Integer.hs"),
    spans_chirho: &[
        (14257, 14301),
        (15876, 15920),
        (17248, 17292),
        (18887, 18923),
        (19403, 19436),
        (19792, 19828),
    ],
    imports_chirho: "",
    primitive_signatures_chirho: true,
};

pub(super) fn manifest_chirho(module_chirho: &str) -> Option<&'static SourceManifestChirho> {
    match module_chirho {
        "GHC.Internal.Data.Tuple" => Some(&TUPLE_CHIRHO),
        NUM_MODULE_CHIRHO => Some(&NUM_CHIRHO),
        INTEGER_MODULE_CHIRHO => Some(&INTEGER_CHIRHO),
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
            if self.primitive_signatures_chirho {
                if self.module_chirho != INTEGER_MODULE_CHIRHO {
                    return Err(NextErrorChirho::new_chirho(
                        NextErrorKindChirho::UnsupportedChirho,
                        "source manifest has no primitive-body provider",
                    ));
                }
                integer_backend_chirho::validate_signature_span_chirho(
                    self.bytes_chirho,
                    start_chirho,
                    end_chirho,
                )?;
            } else {
                boundaries_chirho::validate_chirho(self.bytes_chirho, start_chirho, end_chirho)?;
            }
            source_chirho.push_str(selection_chirho);
            source_chirho.push('\n');
            previous_chirho = end_chirho;
        }
        Ok(source_chirho)
    }
}

#[cfg(test)]
mod integer_tests_chirho;
#[cfg(test)]
mod tests_chirho;
