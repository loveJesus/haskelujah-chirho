// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `sum` and `product` from their solved `Num` proof (row 552). The generic bindings are
//! Int-shaped: `sum (x:xs) = x +# sum xs` from an Int 0, and `product` with `*#`. Over
//! `[Double]`, the interpreter failed on `AddIntChirho`, and both native backends added
//! the Doubles' bits as integers, printing NaN, 0.0 or -1.348e308. An evidenced
//! occurrence, applied or first-class, becomes a fold over its element type's own rows:
//! `foldl $prim_Num_+_K ($prim_Num_fromInteger_K 0)` and
//! `foldl $prim_Num_*_K ($prim_Num_fromInteger_K 1)`. With no evidence, or a missing row,
//! today's path is kept, as it is when a row has no actual body.
//!
//! `max`, `min`, `maximum` and `minimum` from their solved `Ord` proof (row 565). Their
//! generic bindings are Int-shaped too, comparing with `<=#` and `>=#`. Outside Int, Char
//! and Bool, the interpreter failed on `LeIntChirho`, and both native backends picked by
//! comparing heap values as integers: `minimum ["pear", "apple", "zoo"]` was "pear" and
//! `maximum [1, 0 / 0]` was NaN. An evidenced occurrence at a key with a body-backed
//! `Ord.compare` row becomes its compare-backed helper applied to that row, such as
//! `$ord_maximum_by_chirho $prim_Ord_compare_K`.
//! workflow: print-show-evidence-chirho
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho;

use super::DictPassCtxChirho;
use crate::dict_chirho::instance_chirho::structured_equalities_chirho::{
    EQUALITY_CONSUMERS_CHIRHO, INT_SHAPED_KEYS_CHIRHO,
};
use crate::dict_chirho::prelude_chirho::fractional_power_chirho::FRACTIONAL_POWER_DOUBLE_INT_CHIRHO;
use crate::dict_chirho::prelude_chirho::ordering_extrema_chirho::ORDERING_EXTREMA_CHIRHO;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreLitChirho};

impl DictPassCtxChirho {
    /// A `(^^)` reference proved at both `Fractional Double` and `Integral Int`, as the
    /// concrete helper applied to its body-backed reciprocal row. Any absent, open or
    /// unsupported proof pair keeps the canonical named-failure route.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn evidenced_fractional_power_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let name_chirho = self.names_chirho.get(&id_chirho)?;
        if name_chirho.rsplit('.').next() != Some("^^") {
            return None;
        }
        let evidence_chirho = self.reference_evidence_chirho.get(&id_chirho)?;
        let fractional_key_chirho = evidence_chirho
            .iter()
            .find(|(class_chirho, _)| class_chirho == "Fractional")?
            .1
            .as_deref();
        let integral_key_chirho = evidence_chirho
            .iter()
            .find(|(class_chirho, _)| class_chirho == "Integral")?
            .1
            .as_deref();
        if fractional_key_chirho != Some("Double") || integral_key_chirho != Some("Int") {
            return None;
        }
        let divide_chirho =
            self.lookup_dispatch_body_name_id_chirho("$prim_Fractional_/_Double")?;
        let helper_chirho =
            self.lookup_body_backed_name_id_chirho(FRACTIONAL_POWER_DOUBLE_INT_CHIRHO)?;
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(helper_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(divide_chirho)),
        })
    }

    /// `sum` or `product`: the `Num` method its fold steps with and its start.
    fn aggregate_shape_chirho(&self, id_chirho: CoreIdChirho) -> Option<(&'static str, i64)> {
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&id_chirho)?;
        match reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho)
        {
            "sum" => Some(("+", 0)),
            "product" => Some(("*", 1)),
            _ => None,
        }
    }

    /// `foldl step (fromInteger unit)`, an aggregate's fold over one dictionary's rows.
    fn aggregate_fold_chirho(
        &self,
        step_chirho: CoreExprChirho,
        from_integer_chirho: CoreExprChirho,
        unit_chirho: i64,
    ) -> Option<CoreExprChirho> {
        let foldl_id_chirho = self.lookup_compiler_owned_name_id_chirho("foldl")?;
        let start_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(from_integer_chirho),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                unit_chirho,
            ))),
        };
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                arg_chirho: Box::new(step_chirho),
            }),
            arg_chirho: Box::new(start_chirho),
        })
    }

    /// An evidenced `sum` or `product` occurrence as a fold over its element type's
    /// own `Num` rows, or None.
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_aggregate_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let (operator_chirho, unit_chirho) = self.aggregate_shape_chirho(id_chirho)?;
        let (class_name_chirho, ty_key_chirho) = self.occurrence_evidence_chirho.get(&id_chirho)?;
        if class_name_chirho != "Num" {
            return None;
        }
        // Only rows with an actual body count: the global name table can hold
        // dangling placeholder ids, and a generated missing-method body is not
        // the concrete row.
        let operator_id_chirho = self.lookup_dispatch_body_name_id_chirho(&format!(
            "$prim_Num_{operator_chirho}_{ty_key_chirho}"
        ))?;
        let from_integer_id_chirho = self.lookup_dispatch_body_name_id_chirho(&format!(
            "$prim_Num_fromInteger_{ty_key_chirho}"
        ))?;
        self.aggregate_fold_chirho(
            CoreExprChirho::VarChirho(operator_id_chirho),
            CoreExprChirho::VarChirho(from_integer_id_chirho),
            unit_chirho,
        )
    }

    /// An evidenced `sum` or `product` at the enclosing binding's own `Num` key, as the
    /// same fold over that dictionary's methods (row 586). For example, it gives
    /// `foldl ($sel_Num_+ d) ($sel_Num_fromInteger d 0)`, where `d` is the binding's
    /// `$dNum`, or the Num its `$dFractional` holds. The Prelude's `sum` is Int-shaped and
    /// served every instantiation, so at Double the interpreter failed on `AddIntChirho`,
    /// and both natives added the Doubles' bits as integers.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn own_aggregate_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        if self.names_programs_own_binder_chirho(id_chirho) {
            return None;
        }
        let (operator_chirho, unit_chirho) = self.aggregate_shape_chirho(id_chirho)?;
        let (class_name_chirho, ty_key_chirho) = self.occurrence_evidence_chirho.get(&id_chirho)?;
        if class_name_chirho != "Num" || !is_own_dictionary_key_chirho(ty_key_chirho) {
            return None;
        }
        let dict_id_chirho = Self::own_dict_chirho(
            "Num",
            ty_key_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
        )?;
        let selected_chirho = |method_chirho: &str| {
            let (_class_chirho, selector_id_chirho) =
                self.method_selectors_chirho.get(method_chirho)?;
            Some(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(*selector_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
            })
        };
        self.aggregate_fold_chirho(
            selected_chirho(operator_chirho)?,
            selected_chirho("fromInteger")?,
            unit_chirho,
        )
    }

    /// An evidenced `maximum` or `minimum` at the enclosing binding's own `Ord` key, as row
    /// 565's compare-backed fold over that dictionary's `compare` (row 612), such as
    /// `$ord_minimum_by_chirho ($sel_Ord_compare d)`. The Prelude's are Int-shaped, so
    /// inside `Ord a =>` code the interpreter failed on `LeIntChirho`, and both natives
    /// compared heap values as integers. `max` and `min` are Ord's methods there, which the
    /// dictionary route selects.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn own_extremum_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        if self.names_programs_own_binder_chirho(id_chirho) {
            return None;
        }
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&id_chirho)?;
        let base_name_chirho = reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho);
        if !matches!(base_name_chirho, "maximum" | "minimum") {
            return None;
        }
        let (_, helper_chirho) = ORDERING_EXTREMA_CHIRHO
            .iter()
            .find(|(name_chirho, _)| *name_chirho == base_name_chirho)?;
        let (class_name_chirho, ty_key_chirho) = self.occurrence_evidence_chirho.get(&id_chirho)?;
        if class_name_chirho != "Ord" || !is_own_dictionary_key_chirho(ty_key_chirho) {
            return None;
        }
        let dict_id_chirho = Self::own_dict_chirho(
            "Ord",
            ty_key_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
        )?;
        let (_class_chirho, compare_selector_id_chirho) =
            self.method_selectors_chirho.get("compare")?;
        let helper_id_chirho = self.lookup_body_backed_name_id_chirho(helper_chirho)?;
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(helper_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(*compare_selector_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
            }),
        })
    }

    /// An own-key aggregate (row 586) or extremum (row 612), served by the binding's
    /// dictionaries.
    fn own_evidenced_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        self.own_aggregate_chirho(id_chirho, dict_vars_chirho, evidence_classes_chirho)
            .or_else(|| {
                self.own_extremum_chirho(id_chirho, dict_vars_chirho, evidence_classes_chirho)
            })
    }

    /// An evidenced `max`, `min`, `maximum` or `minimum` occurrence as its compare-backed
    /// helper applied to its key's own `Ord.compare` row, or None. Int, Char and Bool keep
    /// their exact Int-shaped bindings, and a key without a body-backed compare row keeps
    /// today's path.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn evidenced_extremum_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&id_chirho)?;
        let base_name_chirho = reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho);
        let (_, helper_chirho) = ORDERING_EXTREMA_CHIRHO
            .iter()
            .find(|(name_chirho, _)| *name_chirho == base_name_chirho)?;
        let (class_name_chirho, ty_key_chirho) = self.occurrence_evidence_chirho.get(&id_chirho)?;
        if class_name_chirho != "Ord" || INT_SHAPED_KEYS_CHIRHO.contains(&ty_key_chirho.as_str()) {
            return None;
        }
        let compare_id_chirho = self
            .lookup_dispatch_body_name_id_chirho(&format!("$prim_Ord_compare_{ty_key_chirho}"))?;
        let helper_id_chirho = self.lookup_body_backed_name_id_chirho(helper_chirho)?;
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(helper_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(compare_id_chirho)),
        })
    }

    /// An evidenced `elem`, `notElem`, `lookup`, `nub`, `group`, `isPrefixOf` or
    /// `isSuffixOf` occurrence as its helper applied to its key's own `Eq.==` row, or None.
    /// Int, Char and Bool keep their exact Int-shaped bindings, and a key without a
    /// body-backed row, or a helper without a body, keeps today's path (row 603).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn evidenced_equality_consumer_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&id_chirho)?;
        let base_name_chirho = reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho);
        let (_, helper_chirho) = EQUALITY_CONSUMERS_CHIRHO
            .iter()
            .find(|(name_chirho, _)| *name_chirho == base_name_chirho)?;
        let (class_name_chirho, ty_key_chirho) = self.occurrence_evidence_chirho.get(&id_chirho)?;
        if class_name_chirho != "Eq" || INT_SHAPED_KEYS_CHIRHO.contains(&ty_key_chirho.as_str()) {
            return None;
        }
        let row_id_chirho = self.existing_eq_row_chirho(ty_key_chirho)?;
        let helper_id_chirho = self.lookup_compiler_owned_name_id_chirho(helper_chirho)?;
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(helper_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(row_id_chirho)),
        })
    }

    /// An evidenced occurrence that becomes the same value applied or not: `sum` and
    /// `product` (row 552), then `max`, `min`, `maximum` and `minimum` (row 565). An
    /// applied `print` is not one; it consumes its proof on its own path.
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_applied_head_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        // A program's own binding keeps its body, whatever evidence it carries (row 593).
        if self.names_programs_own_binder_chirho(id_chirho) {
            return None;
        }
        self.evidenced_fractional_power_chirho(id_chirho)
            .or_else(|| self.evidenced_aggregate_chirho(id_chirho))
            .or_else(|| self.evidenced_extremum_chirho(id_chirho))
            .or_else(|| self.evidenced_equality_consumer_chirho(id_chirho))
    }

    /// Every evidenced Prelude occurrence's replacement: `print` (row 550), then the
    /// aggregates and extrema.
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_value_chirho(&self, id_chirho: CoreIdChirho) -> Option<CoreExprChirho> {
        self.evidenced_print_function_chirho(id_chirho)
            .or_else(|| self.evidenced_applied_head_chirho(id_chirho))
    }

    /// [`Self::evidenced_value_chirho`] where the binding's dictionaries are in scope:
    /// an own-key aggregate (row 586) or extremum (row 612) is served by them.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn evidenced_value_in_scope_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        self.evidenced_value_chirho(id_chirho).or_else(|| {
            self.own_evidenced_chirho(id_chirho, dict_vars_chirho, evidence_classes_chirho)
        })
    }

    /// [`Self::evidenced_applied_head_chirho`] where the binding's dictionaries are in
    /// scope (rows 586 and 612).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn evidenced_applied_head_in_scope_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        self.evidenced_applied_head_chirho(id_chirho).or_else(|| {
            self.own_evidenced_chirho(id_chirho, dict_vars_chirho, evidence_classes_chirho)
        })
    }
}
