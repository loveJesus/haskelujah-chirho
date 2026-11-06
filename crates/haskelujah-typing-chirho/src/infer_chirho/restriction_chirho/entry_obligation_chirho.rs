// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The program entry's obligation (Haskell 2010 section 5): a program's value is its entry,
//! which must be an IO action `IO t` (row 692, gpt_chirho #32531).
//!
//! **Who carries it.** The caller decides the purpose. An executable caller names the entry of
//! the program's root module: the CLI's run and compile, and the all-engine executable test
//! helpers. A value, library or check purpose, and every dependency, names none, so a pure
//! evaluator entry stays a value. `-main-is` is not modelled: the caller's named binding of
//! the root module is the entry, whatever the module is called, and it is never skipped.
//!
//! **When it runs.** After the module's inference and before any final default, so a restricted
//! entry's monad is fixed by this role and never by a default. A missing entry, an entry that
//! is not an action, and an instantiated wanted that no instance can ever satisfy
//! (`Num (IO t)`) are each reported.
//!
//! **A named consumer seam.** Instantiating a polymorphic entry (a signature `Num a => IO a`,
//! or an entry under NoMonomorphismRestriction) proves its IO type and defers the instantiated
//! wanteds. Its Core binding still takes their dictionaries, and the existing entry wrapper
//! does not apply them. Supplying them needs a typed entry instantiation carried to that
//! wrapper. The original polymorphic binding is never changed here.
//! workflow: language-features-chirho/fundep-generalization-chirho

use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{InferCtxChirho, TYPE_MISMATCH_CODE_CHIRHO, UNBOUND_VAR_CODE_CHIRHO};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;

impl InferCtxChirho {
    /// Require the caller's program entry to be an IO action. Returns the substitution the
    /// entry's role applied; `None` asks nothing.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(super) fn require_program_entry_chirho(
        &mut self,
        entry_chirho: Option<&str>,
        final_subst_chirho: &SubstChirho,
    ) -> SubstChirho {
        let Some(entry_name_chirho) = entry_chirho else {
            return SubstChirho::empty_chirho();
        };
        let Some(scheme_chirho) = self.env_chirho.lookup_chirho(entry_name_chirho).cloned() else {
            self.report_program_entry_chirho(
                UNBOUND_VAR_CODE_CHIRHO,
                format!("the program entry `{entry_name_chirho}` is not defined in the program's root module"),
            );
            return SubstChirho::empty_chirho();
        };
        let (entry_ty_chirho, wanteds_chirho) =
            self.instantiate_scheme_parts_chirho(&scheme_chirho);
        let entry_ty_chirho =
            self.normalize_ty_chirho(&final_subst_chirho.apply_ty_chirho(&entry_ty_chirho));
        let action_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IO".to_string())),
            Box::new(self.fresh_var_chirho()),
        );
        let Ok(subst_chirho) = self.unify_normalized_chirho(
            &entry_ty_chirho,
            &action_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ) else {
            self.report_program_entry_chirho(
                TYPE_MISMATCH_CODE_CHIRHO,
                format!(
                    "the program entry `{entry_name_chirho}` must be an IO action `IO t`, but its type is `{entry_ty_chirho}`"
                ),
            );
            return SubstChirho::empty_chirho();
        };
        self.apply_subst_all_chirho(&subst_chirho);
        for wanted_chirho in wanteds_chirho {
            let wanted_chirho = wanted_chirho.mapped_chirho(|ty_chirho| {
                subst_chirho.apply_ty_chirho(&final_subst_chirho.apply_ty_chirho(ty_chirho))
            });
            // A wanted no instance can ever match is the entry's own refusal.
            if !self
                .report_certainly_unsolvable_wanted_chirho(&wanted_chirho, SpanChirho::DUMMY_CHIRHO)
            {
                self.defer_pred_chirho(wanted_chirho, SpanChirho::DUMMY_CHIRHO);
            }
        }
        subst_chirho
    }

    fn report_program_entry_chirho(&mut self, code_chirho: u16, message_chirho: String) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(code_chirho),
                message_chirho,
                SpanChirho::DUMMY_CHIRHO,
            ));
    }
}
