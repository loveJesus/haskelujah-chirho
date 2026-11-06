// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compiler's own Monad declares `m >> k = m >>= \_ -> k` as its default for `>>`, as GHC's
//! does (row 699). Almost every instance writes only `>>=`. A missing `>>` row left a stub in
//! every such dictionary, so G5 never took a one-parameter Monad head as generic and its
//! constructor never found the instance.
//!
//! The authority is the class's own declared default, proved to be the compiler's (gpt_chirho
//! #32268, #32286). G5 coverage and the row itself both require all four of:
//! - Naming's K01 proof that this module's BARE spelling `Monad` names the compiler's Monad. It
//!   is carried in from the backend's builtin class scope, which is fresh for each module. A
//!   proof for a qualified spelling alone never blesses the bare key the class environment uses.
//! - The entry's supported schema: the fields row 694 compares. Equal schemas alone are not
//!   identity.
//! - The entry's own declared default for `>>`, exactly `m >> k = m >>= \_ -> k`, which the row
//!   implements. An absent or changed default authorizes nothing, and no built-in body stands
//!   in for it.
//! - This very instance's body-backed `>>=` row as the donor. A missing-method stub never
//!   donates. An instance with no `>>=` row keeps its named missing-method failure.
//!
//! A source class keeps the default rows its own module desugared, never this body.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho};
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, CompilerClassChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};

/// Whether the class environment's `class_name_chirho` entry has the compiler's supported
/// schema: its parameters, method schemes, superclasses and their templates equal
/// `ClassEnvChirho::seed_standard_chirho`'s, the fields row 694's class authority compares. This
/// alone never proves identity; `monad_default_authorized_chirho` also needs naming's K01 proof.
fn has_supported_schema_chirho(class_env_chirho: &ClassEnvChirho, class_name_chirho: &str) -> bool {
    let mut seeded_chirho = ClassEnvChirho::new_chirho();
    seeded_chirho.seed_standard_chirho();
    match (
        class_env_chirho
            .classes_chirho()
            .get_chirho(class_name_chirho),
        seeded_chirho.classes_chirho().get_chirho(class_name_chirho),
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
/// `>>`. Only a candidate: `monad_default_authorized_chirho` decides. The donor is asked first,
/// so a head without one never builds the seed the schema check compares.
fn monad_default_candidate_chirho(
    class_name_chirho: &str,
    method_name_chirho: &str,
) -> Option<&'static str> {
    (class_name_chirho == "Monad" && method_name_chirho == ">>").then_some(">>=")
}

/// Whether naming proves that this module's bare spelling `class_name_chirho` names the
/// compiler's Monad (K01). Only the bare key the class environment uses counts: a proof for a
/// qualified spelling such as `P.Monad` alone never blesses it.
fn proves_bare_monad_chirho(
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    class_name_chirho: &str,
) -> bool {
    builtin_classes_chirho
        .class_chirho(class_name_chirho)
        .is_some_and(|class_chirho| {
            class_chirho.identity_chirho == CompilerClassChirho::MonadChirho
        })
}

/// Whether `arms_chirho` are exactly `m >> k = m >>= \_ -> k`, the arms the default row
/// implements: one clause of two distinct variable patterns, no `where`, and the unguarded
/// right-hand side `m >>= \_ -> k` with the unqualified `>>=`. Spans are never compared.
fn implements_then_default_chirho(arms_chirho: &[MatchArmChirho]) -> bool {
    let [arm_chirho] = arms_chirho else {
        return false;
    };
    let [
        PatChirho::VarChirho(action_chirho),
        PatChirho::VarChirho(next_chirho),
    ] = arm_chirho.pats_chirho.as_slice()
    else {
        return false;
    };
    let RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
        left_chirho,
        op_chirho,
        right_chirho,
        ..
    }) = &arm_chirho.rhs_chirho
    else {
        return false;
    };
    let ExprChirho::LamChirho {
        pats_chirho,
        body_chirho,
        ..
    } = right_chirho.as_ref()
    else {
        return false;
    };
    let action_name_chirho = action_chirho.full_name_chirho();
    let next_name_chirho = next_chirho.full_name_chirho();
    arm_chirho.where_binds_chirho.is_empty()
        && action_name_chirho != next_name_chirho
        && op_chirho.full_name_chirho() == ">>="
        && matches!(left_chirho.as_ref(), ExprChirho::VarChirho(left_name_chirho)
            if left_name_chirho.full_name_chirho() == action_name_chirho)
        && matches!(pats_chirho.as_slice(), [PatChirho::WildcardChirho(_)])
        && matches!(body_chirho.as_ref(), ExprChirho::VarChirho(body_name_chirho)
            if body_name_chirho.full_name_chirho() == next_name_chirho)
}

/// Whether the compiler's own Monad's declared `>>` default may serve `class_name_chirho` in
/// this module. It needs naming's bare K01 proof, the entry's own declared default being exactly
/// the arms the row implements, and the supported schema, compared last because it builds the
/// seed.
fn monad_default_authorized_chirho(
    class_env_chirho: &ClassEnvChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    class_name_chirho: &str,
) -> bool {
    proves_bare_monad_chirho(builtin_classes_chirho, class_name_chirho)
        && class_env_chirho
            .classes_chirho()
            .get_chirho(class_name_chirho)
            .and_then(|entry_chirho| entry_chirho.defaults_chirho.get(">>"))
            .is_some_and(|arms_chirho| implements_then_default_chirho(arms_chirho))
        && has_supported_schema_chirho(class_env_chirho, class_name_chirho)
}

/// Whether the compiler's own Monad's declared default covers `method_name_chirho` for G5 (row
/// 699). Its donor must be covered (`donor_covered_chirho`, asked first) and the default
/// authorized here. Never for another method or class, a namesake, a missing proof, or an
/// uncovered donor.
pub(in crate::dict_chirho) fn monad_default_covers_chirho(
    class_env_chirho: &ClassEnvChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    class_name_chirho: &str,
    method_name_chirho: &str,
    donor_covered_chirho: impl FnOnce(&'static str) -> bool,
) -> bool {
    monad_default_candidate_chirho(class_name_chirho, method_name_chirho).is_some_and(
        |donor_chirho| {
            donor_covered_chirho(donor_chirho)
                && monad_default_authorized_chirho(
                    class_env_chirho,
                    builtin_classes_chirho,
                    class_name_chirho,
                )
        },
    )
}

impl DictPassCtxChirho {
    /// The default row `$prim_Monad_>>_<key>`: the declared arms `m >> k = m >>= \_ -> k` as
    /// `\m k -> bind m (\_ -> k)`. `bind` is this instance's own body-backed `>>=` row under
    /// `donor_key_chirho`, the key its rows were written under. None unless that row has a real
    /// body and the default is authorized in this module (`monad_default_authorized_chirho`).
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
        let bind_row_chirho = self.existing_keyed_row_chirho(
            &format!("$prim_{class_name_chirho}_{donor_chirho}_"),
            donor_key_chirho,
        )?;
        if !monad_default_authorized_chirho(
            class_env_chirho,
            &self.builtin_classes_chirho,
            class_name_chirho,
        ) {
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
