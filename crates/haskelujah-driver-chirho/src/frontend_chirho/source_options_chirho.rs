// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One module's effective source policy, retaining the real pre-CPP and post-CPP headers.
//! workflow: compiler-pipeline-chirho/error-deferral-chirho

use haskelujah_parser_chirho::pragma_chirho::{
    EffectiveExtensionsChirho, HeaderPragmasChirho, extension_entries_from_options_chirho,
};

use crate::DeferralPolicyChirho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FrontendPolicyChirho {
    pub(crate) deferral_chirho: DeferralPolicyChirho,
    pub(crate) extensions_chirho: EffectiveExtensionsChirho,
    /// The caller's program entry for this module as the program root; never a header's
    /// (row 692). See `FrontendOptionsChirho::program_entry_chirho`.
    pub(crate) program_entry_chirho: Option<String>,
    /// Whether the driver's own standalone route checks this module as the program's default
    /// home unit `main`. Only that route sets it: the public single-source route and the CLI
    /// check entry. Library, project and package callers keep `false`, so their source classes
    /// stay MissingUnit; no caller option can request it.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(crate) standalone_home_chirho: bool,
}

impl FrontendPolicyChirho {
    /// Caller switches precede both real header views. The parser owns edition selection
    /// and extension implications; the existing deferral policy owns deferrable categories.
    pub(crate) fn resolve_chirho(
        caller_options_chirho: &[String],
        headers_chirho: impl IntoIterator<Item = HeaderPragmasChirho>,
    ) -> Self {
        let mut extensions_chirho = extension_entries_from_options_chirho(caller_options_chirho);
        let mut module_options_chirho = Vec::new();
        for header_chirho in headers_chirho {
            extensions_chirho.extend(header_chirho.extensions_chirho);
            module_options_chirho.extend(header_chirho.ghc_options_chirho);
        }
        Self {
            deferral_chirho: DeferralPolicyChirho::resolve_chirho(
                caller_options_chirho,
                &module_options_chirho,
            ),
            extensions_chirho: EffectiveExtensionsChirho::from_entries_chirho(extensions_chirho),
            program_entry_chirho: None,
            standalone_home_chirho: false,
        }
    }
}
