// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The checker's half of do selection: the evidence for the operation a do
//! statement SELECTED.
//!
//! Lowering chose each statement's `>>=`, `>>` or `fail` once and recorded it on
//! the statement, with an origin of its own; Core already resolves exactly that
//! binding. Until now the checker never looked at it at all — it typed a do block
//! by unifying `m a` shapes and emitted no predicate for any operation. So there
//! was no evidence for a do operator anywhere, and nothing to tell the
//! dictionary pass which instance a statement's `>>=` belongs to.
//!
//! This looks the selected binding up, instantiates its ACTUAL scheme — not a
//! manufactured `Monad m`, so a QualifiedDo or RebindableSyntax operation brings
//! its own predicates — and captures those predicates under the selection's own
//! origin, through the same capture every reference uses.
//!
//! Two things it deliberately does NOT do, and both are about not moving a
//! verdict from here:
//!
//! - it pushes no new WANTED constraint. The captured predicates resolve through
//!   unification with the block's monad, so `Monad IO` is recorded without asking
//!   the solver anything new; an obligation the solver could fail on would be a
//!   separate, measured step;
//! - a failed unification is not an error. An operation whose type does not fit
//!   the block's shape — an indexed or graded QualifiedDo bind, whose types change
//!   from statement to statement — simply records NOTHING. No evidence is the safe
//!   answer; wrong evidence would dispatch the wrong instance. The block's typing
//!   itself is unchanged, and it was already permissive about exactly this.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::provenance_chirho::OccurrenceRoleChirho;
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use haskelujah_span_chirho::SpanChirho;

use super::InferCtxChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;

impl InferCtxChirho {
    /// Capture the predicates of one operation a do statement selected, once its
    /// type is shown to fit the block's monad. `monad_chirho` is the block's
    /// monad constructor as the do typing already tracks it.
    pub(super) fn capture_selected_operation_chirho(
        &mut self,
        selected_chirho: &SelectedOperationChirho,
        monad_chirho: &TyChirho,
        subst_chirho: &mut SubstChirho,
        span_chirho: SpanChirho,
    ) {
        let name_chirho = &selected_chirho.name_chirho;
        let Some(scheme_chirho) = self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
            &name_chirho.full_name_chirho(),
            name_chirho.text_chirho(),
        ) else {
            // Not in scope: resolution reports that loudly in its own phase.
            // Nothing is invented here.
            return;
        };
        let (operation_ty_chirho, preds_chirho) =
            self.instantiate_scheme_parts_chirho(&scheme_chirho);
        let expected_chirho =
            self.expected_operation_shape_chirho(selected_chirho.role_chirho, monad_chirho);
        match self.unify_normalized_chirho(
            &subst_chirho.apply_ty_chirho(&operation_ty_chirho),
            &subst_chirho.apply_ty_chirho(&expected_chirho),
            span_chirho,
        ) {
            Ok(unifier_chirho) => {
                *subst_chirho = unifier_chirho.compose_chirho(subst_chirho);
                self.apply_subst_all_chirho(&unifier_chirho);
                self.capture_occurrence_predicates_chirho(name_chirho, &preds_chirho);
            }
            // Does not fit the block's shape: no evidence rather than wrong
            // evidence, and no error from here.
            Err(_) => {}
        }
    }

    /// The type each role's operation has in a block over `monad_chirho`, with
    /// fresh variables for everything else:
    ///
    ///     >>=    m x -> (x -> m y) -> m y
    ///     >>     m x -> m y -> m y
    ///     fail   String -> m y
    fn expected_operation_shape_chirho(
        &mut self,
        role_chirho: OccurrenceRoleChirho,
        monad_chirho: &TyChirho,
    ) -> TyChirho {
        let in_monad_chirho = |inner_chirho: TyChirho| {
            TyChirho::AppChirho(Box::new(monad_chirho.clone()), Box::new(inner_chirho))
        };
        match role_chirho {
            OccurrenceRoleChirho::Bind => {
                let x_chirho = self.fresh_var_chirho();
                let y_chirho = self.fresh_var_chirho();
                TyChirho::fun_chirho(
                    in_monad_chirho(x_chirho.clone()),
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(x_chirho, in_monad_chirho(y_chirho.clone())),
                        in_monad_chirho(y_chirho),
                    ),
                )
            }
            OccurrenceRoleChirho::Then => {
                let x_chirho = self.fresh_var_chirho();
                let y_chirho = self.fresh_var_chirho();
                TyChirho::fun_chirho(
                    in_monad_chirho(x_chirho),
                    TyChirho::fun_chirho(in_monad_chirho(y_chirho.clone()), in_monad_chirho(y_chirho)),
                )
            }
            OccurrenceRoleChirho::Fail => {
                let y_chirho = self.fresh_var_chirho();
                TyChirho::fun_chirho(TyChirho::string_chirho(), in_monad_chirho(y_chirho))
            }
            // Only the three do roles are ever selected by a statement. Anything
            // else is left unconstrained, so it cannot unify into a claim.
            _ => self.fresh_var_chirho(),
        }
    }
}
