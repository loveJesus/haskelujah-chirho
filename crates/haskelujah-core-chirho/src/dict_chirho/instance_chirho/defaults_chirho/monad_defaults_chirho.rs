// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compiler's own Monad defaults `>>` to `\m k -> m >>= \_ -> k`, as GHC's does (row 699).
//! The seeded Monad class declares `>>=` and `>>`, and almost every instance writes only `>>=`. A
//! missing `>>` row left a stub in every such dictionary, so G5 never took a one-parameter Monad
//! head as generic and its constructor never found the instance.
//!
//! The default belongs to the compiler's Monad, never to a class that only shares its spelling.
//! A program's own namesake has its own declaration and only its own declared defaults. The
//! donor is this very instance's `>>=` row, a real body. A missing-method stub never donates, and
//! an instance with no `>>=` row keeps its named missing-method failure.
//!
//! The authority is OPEN (gpt_chirho #32268). Today the entry is compared with the compiler's
//! seeded schema, the comparison row 694's class authority also makes. Schema equality is not
//! identity: row 694 also needs the naming K01 witness. An entry another producer made with the
//! same compared fields passes this check. The control
//! `a_same_schema_namesake_without_a_producer_proof_never_takes_the_default_chirho` exposes that
//! seam, and no product run counts this default until a producer proof reaches Core and fails
//! closed without one.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};

/// Whether the class environment's `class_name_chirho` entry has the compiler's seeded schema:
/// its parameters, method schemes, superclasses and their templates equal
/// `ClassEnvChirho::seed_standard_chirho`'s, the fields row 694's class authority compares. This
/// is NOT a proof that the entry is the compiler's class. An entry another producer made with
/// the same compared fields passes it too (#32268, the open seam in the module docs).
fn is_compiler_owned_class_chirho(
    class_env_chirho: &ClassEnvChirho,
    class_name_chirho: &str,
) -> bool {
    let mut seeded_chirho = ClassEnvChirho::new_chirho();
    seeded_chirho.seed_standard_chirho();
    match (
        class_env_chirho.classes_chirho.get(class_name_chirho),
        seeded_chirho.classes_chirho.get(class_name_chirho),
    ) {
        (Some(entry_chirho), Some(seed_chirho)) => {
            entry_chirho.var_chirho == seed_chirho.var_chirho
                && entry_chirho.extra_vars_chirho == seed_chirho.extra_vars_chirho
                && entry_chirho.methods_chirho == seed_chirho.methods_chirho
                && entry_chirho.supers_chirho == seed_chirho.supers_chirho
                && entry_chirho.superclass_templates_chirho()
                    == seed_chirho.superclass_templates_chirho()
        }
        _ => false,
    }
}

/// The method a default of Monad's `method_name_chirho` reads, by the names alone: `>>=` for
/// `>>`. Only a candidate. The default also needs that row's real body here and the compiler's
/// own Monad, proved last, so a head without the donor never builds the seed the proof compares.
fn monad_default_candidate_chirho(
    class_name_chirho: &str,
    method_name_chirho: &str,
) -> Option<&'static str> {
    (class_name_chirho == "Monad" && method_name_chirho == ">>").then_some(">>=")
}

/// Whether the compiler's own Monad's default covers `method_name_chirho` (G5, row 699): its
/// donor is covered (`donor_covered_chirho`) and the class environment's `Monad` is the
/// compiler's own. Never for another method or class, a namesake, or an uncovered donor.
pub(in crate::dict_chirho) fn monad_default_covers_chirho(
    class_env_chirho: &ClassEnvChirho,
    class_name_chirho: &str,
    method_name_chirho: &str,
    donor_covered_chirho: impl FnOnce(&str) -> bool,
) -> bool {
    monad_default_candidate_chirho(class_name_chirho, method_name_chirho).is_some_and(
        |donor_chirho| {
            donor_covered_chirho(donor_chirho)
                && is_compiler_owned_class_chirho(class_env_chirho, class_name_chirho)
        },
    )
}

impl DictPassCtxChirho {
    /// The default row `$prim_Monad_>>_<key>`, `\m k -> m >>= \_ -> k`, for the compiler's own
    /// Monad when this instance's `>>=` has a body-backed row under `donor_key_chirho`, the key
    /// its rows were written under. None for any other class, namesake or method, or when `>>=`
    /// has no real body.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn monad_default_row_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        method_name_chirho: &str,
        donor_key_chirho: &str,
        prim_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let donor_chirho = monad_default_candidate_chirho(class_name_chirho, method_name_chirho)?;
        let bind_row_chirho = self
            .existing_keyed_row_chirho(&format!("$prim_Monad_{donor_chirho}_"), donor_key_chirho)?;
        if !is_compiler_owned_class_chirho(class_env_chirho, class_name_chirho) {
            return None;
        }
        let any_ty_chirho = TyChirho::int_chirho();
        let action_chirho = self.fresh_binder_chirho("m", any_ty_chirho.clone());
        let next_chirho = self.fresh_binder_chirho("k", any_ty_chirho.clone());
        let ignored_chirho = self.fresh_binder_chirho("ignored", any_ty_chirho.clone());
        let continuation_chirho = CoreExprChirho::LamChirho {
            binder_chirho: ignored_chirho,
            body_chirho: Box::new(Self::var_expr_chirho(&next_chirho)),
        };
        let body_chirho = Self::apply_two_chirho(
            bind_row_chirho,
            Self::var_expr_chirho(&action_chirho),
            continuation_chirho,
        );
        let row_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: row_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![any_ty_chirho.clone(), any_ty_chirho.clone()],
                    any_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: action_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: next_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        Some(row_id_chirho)
    }
}

#[cfg(test)]
#[path = "monad_defaults_tests_chirho.rs"]
mod tests_chirho;
