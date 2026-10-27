// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ord's six methods besides `compare` default as GHC's do (row 612). The seeded Ord class
//! declared only `compare`, so `<`, `<=`, `>`, `>=`, `max` and `min` had no dictionary slot:
//! inside `Ord a =>` code they fell back to Int primops, and an instance's own operators were
//! never read through a dictionary. Ord's layout is now `[Eq | <, <=, >, >=, compare, max, min]`.
//!
//! A slot the instance does not write takes GHC's default, built from the key's real rows:
//! - each operator, its answer at the key (`ordering_operators_chirho.rs`): the Int primop at
//!   Int, Char and Bool, the IEEE primop at Double, else the key's `compare` row read by the
//!   operator's Ordering test;
//! - `max x y = if x <= y then y else x` and `min x y = if x <= y then x else y`, through the
//!   key's `<=` answer, which is the instance's own `<=` where it writes one.
//!
//! A missing-method stub never donates: a key whose `compare` has no real body keeps each
//! operator's named failure. `compare`'s own default, from `==` and `<=`, is row 626's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use super::ordering_operators_chirho::ORDERING_OPERATORS_CHIRHO;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};

/// The Ord operator, with its Int primop, whose answer `method_name_chirho`'s default reads:
/// the operator itself, or `<=` for `max` and `min`. None for `compare` and any other class.
fn ordering_default_operator_chirho(
    class_name_chirho: &str,
    method_name_chirho: &str,
) -> Option<(&'static str, &'static str)> {
    let operator_chirho = match (class_name_chirho, method_name_chirho) {
        ("Ord", "max" | "min") => "<=",
        ("Ord", method_chirho) => method_chirho,
        _ => return None,
    };
    ORDERING_OPERATORS_CHIRHO
        .into_iter()
        .find(|(written_chirho, _)| *written_chirho == operator_chirho)
}

impl DictPassCtxChirho {
    /// The default row `$prim_Ord_<method>_<key>` for an Ord method besides `compare`, read
    /// from the rows written under `donor_key_chirho`; None for any other class or method,
    /// or when the key has no answer for the operator the default reads.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn ordering_default_row_chirho(
        &mut self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        donor_key_chirho: &str,
        prim_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let operator_chirho =
            ordering_default_operator_chirho(class_name_chirho, method_name_chirho)?;
        let answer_chirho = self.ordering_answer_chirho(operator_chirho, donor_key_chirho)?;

        let any_ty_chirho = TyChirho::int_chirho();
        let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", any_ty_chirho.clone());
        let answered_chirho = answer_chirho.applied_chirho(
            Self::var_expr_chirho(&x_chirho),
            Self::var_expr_chirho(&y_chirho),
        );
        let (body_chirho, result_ty_chirho) = match method_name_chirho {
            "max" | "min" => {
                // `x <= y` picks `y` for `max` and `x` for `min`; otherwise the other one.
                let (at_most_chirho, otherwise_chirho) = if method_name_chirho == "max" {
                    (&y_chirho, &x_chirho)
                } else {
                    (&x_chirho, &y_chirho)
                };
                let at_most_ty_chirho = TyChirho::bool_chirho();
                let decided_chirho = self.fresh_binder_chirho("$ord_default", at_most_ty_chirho);
                let body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(answered_chirho),
                    bind_chirho: decided_chirho,
                    result_ty_chirho: any_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: Self::var_expr_chirho(at_most_chirho),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: Self::var_expr_chirho(otherwise_chirho),
                        },
                    ],
                };
                (body_chirho, any_ty_chirho.clone())
            }
            _ => (answered_chirho, TyChirho::bool_chirho()),
        };

        let row_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: row_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![any_ty_chirho.clone(), any_ty_chirho],
                    result_ty_chirho,
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
}
