// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ord's six methods besides `compare` receive dictionary slots and body-backed defaults in row
//! 612. The seeded Ord class declared only `compare`, so `<`, `<=`, `>`, `>=`, `max` and `min`
//! had no dictionary slot:
//! inside `Ord a =>` code they fell back to Int primops, and an instance's own operators were
//! never read through a dictionary. Ord's layout is now `[Eq | <, <=, >, >=, compare, max, min]`.
//!
//! A slot the instance does not write takes GHC's default (GHC.Classes) from the key's real rows:
//! - `<=` reads the key's `compare` row through its Ordering test;
//! - `x < y = not (y <= x)`, `x > y = not (x <= y)` and `x >= y = y <= x`, through the key's
//!   `<=`, which is the instance's own `<=` where it writes one (row 626; row 612 had left these
//!   three compare-backed);
//! - `max x y = if x <= y then y else x` and `min x y = if x <= y then x else y`, through the
//!   key's `<=` answer;
//! - `compare x y = if x == y then EQ else if x <= y then LT else GT`, through the Eq
//!   superclass's real `==` and the instance's own `<=`, so an instance that writes only `<=`
//!   orders everywhere (row 626).
//!
//! A missing-method stub never donates: a key with neither a real `compare` nor its own `<=`
//! keeps each slot's named failure, where GHC's defaults call each other forever.
//!
//! `max` and `min` are read through these rows at every served key since row 627: a concrete
//! key without a dictionary, such as `Maybe Int` or a tuple, gets its `max` and `min` rows the
//! same way, and `maximum` and `minimum` fold them.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::BTreeSet;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use super::ordering_operators_chirho::ORDERING_OPERATORS_CHIRHO;
use crate::dict_chirho::type_keys_chirho::canonical_type_key_chirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};

/// The Ord operator, with its Int primop, whose answer `method_name_chirho`'s default reads:
/// the operator itself, or `<=` for `max`, `min` and `compare`. None for any other class.
fn ordering_default_operator_chirho(
    class_name_chirho: &str,
    method_name_chirho: &str,
) -> Option<(&'static str, &'static str)> {
    let operator_chirho = match (class_name_chirho, method_name_chirho) {
        ("Ord", "max" | "min" | "compare") => "<=",
        ("Ord", method_chirho) => method_chirho,
        _ => return None,
    };
    ORDERING_OPERATORS_CHIRHO
        .into_iter()
        .find(|(written_chirho, _)| *written_chirho == operator_chirho)
}

/// `case scrutinee of True -> when_true; _ -> otherwise`.
fn if_then_else_chirho(
    scrutinee_chirho: CoreExprChirho,
    decided_chirho: BinderChirho,
    result_ty_chirho: TyChirho,
    when_true_chirho: CoreExprChirho,
    otherwise_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(scrutinee_chirho),
        bind_chirho: decided_chirho,
        result_ty_chirho,
        alts_chirho: vec![
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("True".to_string()),
                binders_chirho: vec![],
                rhs_chirho: when_true_chirho,
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: otherwise_chirho,
            },
        ],
    }
}

impl DictPassCtxChirho {
    /// The key's `$prim_Ord_<method>_<key>` row under the proof's spelling, its canonical
    /// spelling or with `String` spelled out. A missing-method stub counts, so a key whose
    /// slot has no real body fails by name (row 627).
    pub(in crate::dict_chirho) fn ordering_method_row_chirho(
        &self,
        method_name_chirho: &str,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let canonical_chirho = canonical_type_key_chirho(key_chirho);
        let spelled_chirho = Self::string_spelled_out_chirho(&canonical_chirho);
        [key_chirho.to_string(), canonical_chirho, spelled_chirho]
            .iter()
            .find_map(|spelling_chirho| {
                self.lookup_body_backed_name_id_chirho(&format!(
                    "$prim_Ord_{method_name_chirho}_{spelling_chirho}"
                ))
            })
    }

    /// A `max` or `min` row, GHC's default through the key's `<=`, for each concrete key a
    /// `max`, `min`, `maximum` or `minimum` occurrence was proved at that has none, such as
    /// the `Maybe` and tuple keys row 572 composes compare rows for (row 627). Runs after
    /// those rows exist; a key with no `<=` answer gets no row.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_evidenced_extremum_rows_chirho(&mut self) {
        let wanted_chirho = self
            .occurrence_evidence_chirho
            .iter()
            .filter(|(id_chirho, (class_name_chirho, key_chirho))| {
                class_name_chirho == "Ord"
                    && !is_own_dictionary_key_chirho(key_chirho)
                    && !self.names_programs_own_binder_chirho(**id_chirho)
            })
            .filter_map(|(id_chirho, (_, key_chirho))| {
                let (name_chirho, _) = self.method_occurrence_canon_chirho.get(id_chirho)?;
                let method_chirho = match name_chirho.rsplit('.').next()? {
                    "max" | "maximum" => "max",
                    "min" | "minimum" => "min",
                    _ => return None,
                };
                Some((method_chirho, key_chirho.clone()))
            })
            .collect::<BTreeSet<_>>();
        for (method_chirho, key_chirho) in wanted_chirho {
            if self
                .ordering_method_row_chirho(method_chirho, &key_chirho)
                .is_none()
            {
                self.ordering_default_row_chirho(
                    "Ord",
                    method_chirho,
                    &key_chirho,
                    &format!("$prim_Ord_{method_chirho}_{key_chirho}"),
                );
            }
        }
    }

    /// The key's body-backed `Eq.==` row: the instance's own or derived one, else row 606's
    /// default from its own `/=`. None when Eq has neither real body at the key.
    fn real_equality_row_chirho(&mut self, key_chirho: &str) -> Option<CoreIdChirho> {
        self.existing_keyed_row_chirho("$prim_Eq_==_", key_chirho)
            .or_else(|| {
                self.equality_default_row_chirho(
                    "Eq",
                    "==",
                    key_chirho,
                    &format!("$prim_Eq_==_{key_chirho}"),
                )
            })
    }

    /// The default row `$prim_Ord_<method>_<key>` for an Ord method the instance does not
    /// write, read from the rows written under `donor_key_chirho`; None for any other class
    /// or method, or when the key has no answer for the `<=` or operator the default reads,
    /// or, for `compare`, no real `==`.
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
        let equality_chirho = if method_name_chirho == "compare" {
            Some(self.real_equality_row_chirho(donor_key_chirho)?)
        } else {
            None
        };

        let any_ty_chirho = TyChirho::int_chirho();
        let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", any_ty_chirho.clone());
        let answered_chirho = answer_chirho.applied_chirho(
            Self::var_expr_chirho(&x_chirho),
            Self::var_expr_chirho(&y_chirho),
        );
        let (body_chirho, result_ty_chirho) = match (method_name_chirho, equality_chirho) {
            ("compare", Some(equality_chirho)) => {
                // `if x == y then EQ else if x <= y then LT else GT`.
                let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
                let equal_chirho = self.fresh_binder_chirho("$ord_equal", TyChirho::bool_chirho());
                let at_most_chirho =
                    self.fresh_binder_chirho("$ord_at_most", TyChirho::bool_chirho());
                let ordered_chirho = if_then_else_chirho(
                    answered_chirho,
                    at_most_chirho,
                    ordering_ty_chirho.clone(),
                    Self::nullary_con_chirho("LT"),
                    Self::nullary_con_chirho("GT"),
                );
                let body_chirho = if_then_else_chirho(
                    Self::apply_two_chirho(
                        equality_chirho,
                        Self::var_expr_chirho(&x_chirho),
                        Self::var_expr_chirho(&y_chirho),
                    ),
                    equal_chirho,
                    ordering_ty_chirho.clone(),
                    Self::nullary_con_chirho("EQ"),
                    ordered_chirho,
                );
                (body_chirho, ordering_ty_chirho)
            }
            ("max" | "min", _) => {
                // `x <= y` picks `y` for `max` and `x` for `min`; otherwise the other one.
                let (at_most_chirho, otherwise_chirho) = if method_name_chirho == "max" {
                    (&y_chirho, &x_chirho)
                } else {
                    (&x_chirho, &y_chirho)
                };
                let decided_chirho =
                    self.fresh_binder_chirho("$ord_default", TyChirho::bool_chirho());
                let body_chirho = if_then_else_chirho(
                    answered_chirho,
                    decided_chirho,
                    any_ty_chirho.clone(),
                    Self::var_expr_chirho(at_most_chirho),
                    Self::var_expr_chirho(otherwise_chirho),
                );
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
