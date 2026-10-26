// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `sum` and `product` from their solved `Num` proof (row 552). The generic bindings are
//! Int-shaped: `sum (x:xs) = x +# sum xs` from an Int 0, and `product` with `*#`. Over
//! `[Double]`, the interpreter failed on `AddIntChirho`, and both native backends added
//! the Doubles' bits as integers, printing NaN, 0.0 or -1.348e308. An evidenced
//! occurrence, applied or first-class, becomes a fold over its element type's own rows:
//! `foldl $prim_Num_+_K ($prim_Num_fromInteger_K 0)` and
//! `foldl $prim_Num_*_K ($prim_Num_fromInteger_K 1)`. With no evidence, or a missing row,
//! today's path is kept, as it is when a row has no actual body.
//! workflow: print-show-evidence-chirho

use super::DictPassCtxChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreLitChirho};

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

    /// Every evidenced Prelude occurrence's replacement: `print` (row 550), then `sum`
    /// and `product` (row 552).
    /// workflow: print-show-evidence-chirho
    pub(super) fn evidenced_value_chirho(&self, id_chirho: CoreIdChirho) -> Option<CoreExprChirho> {
        self.evidenced_print_function_chirho(id_chirho)
            .or_else(|| self.evidenced_aggregate_chirho(id_chirho))
    }
}
