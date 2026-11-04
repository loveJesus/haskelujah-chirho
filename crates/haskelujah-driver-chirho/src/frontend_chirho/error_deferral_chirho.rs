// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's three independent error-deferral switches, decided from real options (row 695).
//!
//! `-fdefer-type-errors` turns on all three: type errors, typed holes and out-of-scope
//! variables. `-fno-defer-type-errors` turns off only the general switch.
//! `-fdefer-typed-holes`, `-fdefer-out-of-scope-variables` and their `-fno-` forms change only
//! their own category. Options apply left to right: the caller's options first, then the
//! module header's `OPTIONS_GHC` pragmas, which the parser reads from real pragma tokens. A
//! flag spelled in a comment, a string literal or a pragma after the header is not an option.
//!
//! Only the inference errors the typing crate names as deferrable can become warnings. Name
//! resolution, kind, validity, structural and uncoded errors stay fatal under every switch.
//! Deferring makes the module compile; nothing here claims a deferred error behaves at run
//! time as GHC's deferred runtime error would.
//! workflow: compiler-pipeline-chirho/error-deferral-chirho

use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, SeverityChirho};
use haskelujah_typing_chirho::infer_chirho::{
    DEFERRABLE_TYPE_ERROR_CODES_CHIRHO, OUT_OF_SCOPE_VARIABLE_ERROR_CODE_CHIRHO,
    TYPED_HOLE_ERROR_CODE_CHIRHO,
};

/// GHC options a caller passes ahead of the module's own `OPTIONS_GHC` pragmas, in
/// command-line order. The entry points that take no options pass none.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FrontendOptionsChirho {
    pub ghc_options_chirho: Vec<String>,
}

impl FrontendOptionsChirho {
    /// The options in the order a command line would give them.
    pub fn from_ghc_options_chirho<OptionsChirho, OptionChirho>(
        options_chirho: OptionsChirho,
    ) -> Self
    where
        OptionsChirho: IntoIterator<Item = OptionChirho>,
        OptionChirho: Into<String>,
    {
        Self {
            ghc_options_chirho: options_chirho.into_iter().map(Into::into).collect(),
        }
    }
}

/// Which inference error categories become warnings instead of stopping the module.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeferralPolicyChirho {
    pub type_errors_chirho: bool,
    pub typed_holes_chirho: bool,
    pub out_of_scope_variables_chirho: bool,
}

impl DeferralPolicyChirho {
    /// The caller's options, then the module header's, each in its own order.
    pub fn resolve_chirho(caller_chirho: &[String], module_header_chirho: &[String]) -> Self {
        let mut policy_chirho = Self::default();
        for option_chirho in caller_chirho.iter().chain(module_header_chirho) {
            policy_chirho.apply_option_chirho(option_chirho);
        }
        policy_chirho
    }

    /// Applies one option word. A word that is not a deferral switch changes nothing.
    pub fn apply_option_chirho(&mut self, option_chirho: &str) {
        match option_chirho {
            "-fdefer-type-errors" => {
                self.type_errors_chirho = true;
                self.typed_holes_chirho = true;
                self.out_of_scope_variables_chirho = true;
            }
            "-fno-defer-type-errors" => self.type_errors_chirho = false,
            "-fdefer-typed-holes" => self.typed_holes_chirho = true,
            "-fno-defer-typed-holes" => self.typed_holes_chirho = false,
            "-fdefer-out-of-scope-variables" => self.out_of_scope_variables_chirho = true,
            "-fno-defer-out-of-scope-variables" => self.out_of_scope_variables_chirho = false,
            _ => {}
        }
    }

    /// Whether this inference error becomes a warning: its code must name a deferrable
    /// category, and that category's switch must be on.
    fn defers_inference_error_chirho(&self, diagnostic_chirho: &DiagnosticChirho) -> bool {
        match diagnostic_chirho.code_chirho {
            Some(code_chirho) if code_chirho == TYPED_HOLE_ERROR_CODE_CHIRHO => {
                self.typed_holes_chirho
            }
            Some(code_chirho) if code_chirho == OUT_OF_SCOPE_VARIABLE_ERROR_CODE_CHIRHO => {
                self.out_of_scope_variables_chirho
            }
            Some(code_chirho) => {
                self.type_errors_chirho && DEFERRABLE_TYPE_ERROR_CODES_CHIRHO.contains(&code_chirho)
            }
            None => false,
        }
    }

    /// Settles inference's diagnostics. When every error among them is one this policy
    /// defers, those errors become warnings and the module goes on. Otherwise nothing changes
    /// and the caller reports the whole bundle as fatal.
    pub(crate) fn settle_inference_chirho(
        &self,
        diagnostics_chirho: &mut DiagnosticBundleChirho,
    ) -> bool {
        if !diagnostics_chirho.has_errors_chirho() {
            return true;
        }
        if !diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .filter(|diagnostic_chirho| diagnostic_chirho.is_error_chirho())
            .all(|diagnostic_chirho| self.defers_inference_error_chirho(diagnostic_chirho))
        {
            return false;
        }
        let settled_chirho = std::mem::take(diagnostics_chirho)
            .into_vec_chirho()
            .into_iter()
            .map(|mut diagnostic_chirho| {
                if diagnostic_chirho.is_error_chirho() {
                    diagnostic_chirho.severity_chirho = SeverityChirho::WarningChirho;
                }
                diagnostic_chirho
            })
            .collect();
        *diagnostics_chirho = DiagnosticBundleChirho::new_chirho(settled_chirho);
        true
    }
}
