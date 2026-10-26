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

use super::DictPassCtxChirho;
use crate::dict_chirho::prelude_chirho::ordering_extrema_chirho::ORDERING_EXTREMA_CHIRHO;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreLitChirho};

/// Keys whose Int-shaped `max`, `min`, `maximum` and `minimum` are already exact.
const INT_SHAPED_ORD_KEYS_CHIRHO: [&str; 3] = ["Int", "Char", "Bool"];

impl DictPassCtxChirho {
    /// An evidenced `sum` or `product` occurrence as a fold over its element type's
    /// own `Num` rows, or None.
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_aggregate_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&id_chirho)?;
        let (operator_chirho, unit_chirho) = match reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho)
        {
            "sum" => ("+", 0),
            "product" => ("*", 1),
            _ => return None,
        };
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
        let foldl_id_chirho = self.lookup_body_backed_name_id_chirho("foldl")?;
        let start_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(from_integer_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                unit_chirho,
            ))),
        };
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(operator_id_chirho)),
            }),
            arg_chirho: Box::new(start_chirho),
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
        if class_name_chirho != "Ord"
            || INT_SHAPED_ORD_KEYS_CHIRHO.contains(&ty_key_chirho.as_str())
        {
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

    /// An evidenced occurrence that becomes the same value applied or not: `sum` and
    /// `product` (row 552), then `max`, `min`, `maximum` and `minimum` (row 565). An
    /// applied `print` is not one; it consumes its proof on its own path.
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_applied_head_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        self.evidenced_aggregate_chirho(id_chirho)
            .or_else(|| self.evidenced_extremum_chirho(id_chirho))
    }

    /// Every evidenced Prelude occurrence's replacement: `print` (row 550), then the
    /// aggregates and extrema.
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_value_chirho(&self, id_chirho: CoreIdChirho) -> Option<CoreExprChirho> {
        self.evidenced_print_function_chirho(id_chirho)
            .or_else(|| self.evidenced_applied_head_chirho(id_chirho))
    }
}
