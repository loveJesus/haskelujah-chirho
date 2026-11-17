// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual IO wrappers and opaque source layout provenance.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
pub(super) const SYSTEM_IO_CHIRHO: &str = "GHC.Internal.System.IO";
pub(super) const HANDLE_TEXT_CHIRHO: &str = "GHC.Internal.IO.Handle.Text";
pub(super) const STD_HANDLES_CHIRHO: &str = "GHC.Internal.IO.StdHandles";
pub(super) const HANDLE_TYPES_CHIRHO: &str = "GHC.Internal.IO.Handle.Types";
const SYSTEM_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    explicit_forall_chirho: false,
    ghc_version_chirho: "9.14.1",
    exports_chirho: Some("print, putStr, putStrLn"),
    cpp_chirho: Some(cpp_chirho::SourceCppChirho {
        header_span_chirho: (530, 5486),
        include_premise_chirho: false,
    }),
    module_chirho: "GHC.Internal.System.IO",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/System/IO.hs",
    sha256_chirho: "f4487f29d520a7e094ddb412529076c1ec78cd5d809bcf56a60504548c3d1f58",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/system-chirho/IO.hs"),
    spans_chirho: &[(7669, 7740), (7906, 7979), (8880, 8957)],
    imports_chirho: "import GHC.Internal.Base (String)\nimport GHC.Internal.Show (Show(..))\nimport GHC.Internal.IO.Handle.Text (hPutStr, hPutStrLn)\nimport GHC.Internal.IO.StdHandles (stdout)\n",
    primitive_signatures_chirho: false,
    wired_imports_chirho: &[WiredNameChirho::IOChirho],
};
const TEXT_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    explicit_forall_chirho: false,
    ghc_version_chirho: "9.14.1",
    exports_chirho: Some("hPutStr, hPutStrLn"),
    cpp_chirho: Some(cpp_chirho::SourceCppChirho {
        header_span_chirho: (1010, 1304),
        include_premise_chirho: false,
    }),
    module_chirho: "GHC.Internal.IO.Handle.Text",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/IO/Handle/Text.hs",
    sha256_chirho: "4b5263380b09431adbddb9cdbad9dd3447baab6938418798bc5d0d0f866a59d4",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/io-chirho/handle-chirho/Text.hs"),
    spans_chirho: &[(24369, 24453), (24620, 24707), (24734, 24780)],
    imports_chirho: "import GHC.Internal.Base (String)\nimport GHC.Internal.IO.Handle.Types (Handle)\n",
    primitive_signatures_chirho: true,
    wired_imports_chirho: &[
        WiredNameChirho::IOChirho,
        WiredNameChirho::BoolChirho,
        WiredNameChirho::TrueChirho,
        WiredNameChirho::FalseChirho,
    ],
};
const STD_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    explicit_forall_chirho: false,
    ghc_version_chirho: "9.14.1",
    exports_chirho: Some("stdout"),
    cpp_chirho: Some(cpp_chirho::SourceCppChirho {
        header_span_chirho: (618, 803),
        include_premise_chirho: false,
    }),
    module_chirho: "GHC.Internal.IO.StdHandles",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/IO/StdHandles.hs",
    sha256_chirho: "f9400f4ea31ec3f8d4961eca17790e3365dd535ebc614395445ba9ba73081be1",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/io-chirho/StdHandles.hs"),
    spans_chirho: &[(1391, 1408)],
    imports_chirho: "import GHC.Internal.IO.Handle.Types (Handle)\n",
    primitive_signatures_chirho: true,
    wired_imports_chirho: &[],
};
const HANDLE_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    explicit_forall_chirho: false,
    ghc_version_chirho: "9.14.1",
    exports_chirho: Some("Handle"),
    cpp_chirho: Some(cpp_chirho::SourceCppChirho {
        header_span_chirho: (956, 1384),
        include_premise_chirho: false,
    }),
    module_chirho: "GHC.Internal.IO.Handle.Types",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/IO/Handle/Types.hs",
    sha256_chirho: "4c719b6081b5e689219380974f72ed294b231521312f5b9a53bd5202019c9c35",
    bytes_chirho: include_str!(
        "vendor-chirho/ghc-internal-chirho/io-chirho/handle-chirho/Types.hs"
    ),
    spans_chirho: &[(3824, 4462)],
    imports_chirho: "",
    primitive_signatures_chirho: false,
    wired_imports_chirho: &[],
};
const TYPES_CHIRHO: SourceManifestChirho = SourceManifestChirho {
    explicit_forall_chirho: false,
    ghc_version_chirho: "9.14.1",
    exports_chirho: Some("IO"),
    cpp_chirho: None,
    module_chirho: "GHC.Internal.Types",
    upstream_path_chirho: "libraries/ghc-internal/src/GHC/Internal/Types.hs",
    sha256_chirho: "a8fe6ab5c7a84be9b0710078b549aa63e511e7cb12d056cef91403d05ca6d2bb",
    bytes_chirho: include_str!("vendor-chirho/ghc-internal-chirho/Types.hs"),
    spans_chirho: &[],
    imports_chirho: "",
    primitive_signatures_chirho: false,
    wired_imports_chirho: &[],
};
pub(super) const FACADES_CHIRHO: &[&SourceManifestChirho] =
    &[&SYSTEM_CHIRHO, &TEXT_CHIRHO, &STD_CHIRHO, &HANDLE_CHIRHO];
/// Buffer semantics are source facts, not a target-specific runtime guess.
pub(super) fn validate_buffer_chirho() -> Result<(), NextErrorChirho> {
    let bytes_chirho =
        include_str!("vendor-chirho/ghc-internal-chirho/io-chirho/handle-chirho/Internals.hs");
    verify_source_chirho(
        bytes_chirho,
        "bc5fd5b2c19acd0e7988183dc93c5f14922a4d4f59f8c0ffe6de92e61be295cd",
        "libraries/ghc-internal/src/GHC/Internal/IO/Handle/Internals.hs",
        "9.14.1",
    )?;
    let selection_chirho = bytes_chirho.get(19422..19507).ok_or_else(|| {
        NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "missing source char-buffer declaration",
        )
    })?;
    let expected_chirho = format!(
        "dEFAULT_CHAR_BUFFER_SIZE :: Int\ndEFAULT_CHAR_BUFFER_SIZE = {} -- 8k/sizeof(HsChar)\n",
        haskelujah_runtime_chirho::io_chirho::CHAR_BUFFER_SIZE_CHIRHO
    );
    if selection_chirho != expected_chirho || bytes_chirho.as_bytes().get(19421) != Some(&b'\n') {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "source/runtime char-buffer contract drift",
        ));
    }
    Ok(())
}
pub(super) fn manifest_chirho(module_chirho: &str) -> Option<&'static SourceManifestChirho> {
    match module_chirho {
        SYSTEM_IO_CHIRHO => Some(&SYSTEM_CHIRHO),
        HANDLE_TEXT_CHIRHO => Some(&TEXT_CHIRHO),
        STD_HANDLES_CHIRHO => Some(&STD_CHIRHO),
        HANDLE_TYPES_CHIRHO => Some(&HANDLE_CHIRHO),
        "GHC.Internal.Types" => Some(&TYPES_CHIRHO),
        _ => None,
    }
}
