// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Eq's two methods default to each other's negation, as GHC's do (row 606). The seeded Eq
//! class declared only `==`, so a program's own `(/=)` row was never read. Infix `/=` was
//! rewritten to `not (== …)` with an evidence-free `==`, and a section or first-class `(/=)`
//! had no route at all. Eq's layout is now `[/=, ==]`.
//!
//! A key with only one method's row takes the other as `\x y -> not (row x y)`. The donor
//! must be a real body: a missing-method stub never donates. A key with neither real body
//! keeps its named missing-method failure, and no pair of defaults is made from two stubs.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::BTreeSet;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};

/// Each Eq method and the method whose negation is its default.
const EQUALITY_DEFAULTS_CHIRHO: [(&str, &str); 2] = [("/=", "=="), ("==", "/=")];

/// The Eq method whose negation is `method_name_chirho`'s default, for a method of Eq.
pub(in crate::dict_chirho) fn equality_counterpart_chirho(
    class_name_chirho: &str,
    method_name_chirho: &str,
) -> Option<&'static str> {
    (class_name_chirho == "Eq")
        .then(|| {
            EQUALITY_DEFAULTS_CHIRHO
                .iter()
                .find(|(method_chirho, _)| *method_chirho == method_name_chirho)
                .map(|(_, counterpart_chirho)| *counterpart_chirho)
        })
        .flatten()
}

impl DictPassCtxChirho {
    /// The default row `$prim_Eq_<method>_<key>`, `\x y -> not (counterpart x y)`, when the
    /// counterpart method has a body-backed row under `donor_key_chirho`, the key its rows
    /// were written under; None for any other class or method, or when the counterpart has
    /// no real body.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn equality_default_row_chirho(
        &mut self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        donor_key_chirho: &str,
        prim_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let counterpart_chirho =
            equality_counterpart_chirho(class_name_chirho, method_name_chirho)?;
        let counterpart_row_chirho = self.existing_keyed_row_chirho(
            &format!("$prim_Eq_{counterpart_chirho}_"),
            donor_key_chirho,
        )?;
        let any_ty_chirho = TyChirho::int_chirho();
        let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", any_ty_chirho.clone());
        let answer_chirho = self.fresh_binder_chirho("$eq_default", TyChirho::bool_chirho());
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(Self::apply_two_chirho(
                counterpart_row_chirho,
                Self::var_expr_chirho(&x_chirho),
                Self::var_expr_chirho(&y_chirho),
            )),
            bind_chirho: answer_chirho,
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: Self::nullary_con_chirho("False"),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: Self::nullary_con_chirho("True"),
                },
            ],
        };
        let row_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: row_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![any_ty_chirho.clone(), any_ty_chirho],
                    TyChirho::bool_chirho(),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        Some(row_id_chirho)
    }

    /// A `$prim_Eq_/=_K` row for each concrete key a `/=` occurrence was proved at that has
    /// no dictionary to carry one, such as the `Maybe`, list and tuple keys row 603 composes
    /// `==` rows for. Runs after those rows exist.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_evidenced_inequality_rows_chirho(&mut self) {
        let keys_chirho = self
            .occurrence_evidence_chirho
            .iter()
            .filter(|(id_chirho, (class_name_chirho, key_chirho))| {
                class_name_chirho == "Eq"
                    && !is_own_dictionary_key_chirho(key_chirho)
                    && self
                        .method_occurrence_canon_chirho
                        .get(id_chirho)
                        .is_some_and(|(name_chirho, _)| {
                            name_chirho.rsplit('.').next() == Some("/=")
                        })
            })
            .map(|(_, (_, key_chirho))| key_chirho.clone())
            .collect::<BTreeSet<_>>();
        for key_chirho in keys_chirho {
            let row_name_chirho = format!("$prim_Eq_/=_{key_chirho}");
            if self
                .lookup_body_backed_name_id_chirho(&row_name_chirho)
                .is_none()
            {
                self.equality_default_row_chirho("Eq", "/=", &key_chirho, &row_name_chirho);
            }
        }
    }
}
