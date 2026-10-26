// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Double operands at the desugarer's Int primops (row 547). Infix and section
//! `<`, `<=`, `>` and `>=` become `<#`, `<=#`, `>#` and `>=#`, and a prefix minus
//! on a non-float-literal operand becomes `negate#`, before any type is known. At
//! Double those treated IEEE bits as integers: the interpreter refused them, and
//! the native backends ordered negative Doubles backwards. Where an operand's
//! type key is Double, the primop becomes the float one: the ordered comparisons
//! are GHC's `<##`, `<=##`, `>##` and `>=##` (a NaN on either side gives False),
//! and negation flips only the sign. Sections of `+`, `-` and `*` are blind the same
//! way (`(+ 0.5)` becomes `\x -> +# x 0.5`), and take `+.#`, `-.#` and `*.#` when
//! their operand keys as Double (row 554). Sections of `/` already take the
//! Fractional evidence path, so no `/#` reaches this table.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use super::DictPassCtxChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

/// The float primop standing for an Int primop the desugarer chose blind. The
/// type-keyed rebuild in `method_args_chirho.rs` reads the same table (row 553).
pub(super) fn double_counterpart_chirho(name_chirho: &str) -> Option<&'static str> {
    match name_chirho {
        "<#" => Some("<.#"),
        "<=#" => Some("<=.#"),
        ">#" => Some(">.#"),
        ">=#" => Some(">=.#"),
        "negate#" => Some("negateFloat#"),
        "+#" => Some("+.#"),
        "-#" => Some("-.#"),
        "*#" => Some("*.#"),
        _ => None,
    }
}

impl DictPassCtxChirho {
    /// The float primop an Int ordering or negation primop becomes when one of
    /// its operands is known at Double; `None` keeps the primop as it is.
    pub(super) fn primop_at_double_chirho(
        &self,
        name_chirho: &str,
        args_chirho: &[CoreExprChirho],
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<&'static str> {
        let float_chirho = double_counterpart_chirho(name_chirho)?;
        args_chirho
            .iter()
            .any(|arg_chirho| {
                self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                    .is_some_and(|key_chirho| key_chirho == "Double")
            })
            .then_some(float_chirho)
    }
}
