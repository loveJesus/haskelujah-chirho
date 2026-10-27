// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compare route, the first brick of the Ord lane (row 551, gpt_chirho #28919). The
//! desugarer turns infix and section `<`, `<=`, `>` and `>=` into Int primops before any
//! type is known, which met Strings, lists, derived values and Orderings as heap values or
//! C strings. When an operand's concrete key has a body-backed `Ord.compare` row, found by
//! the same resolver compare dispatch trusts, the primop becomes one `compare a b` read by
//! the matching Ordering test. Int, Char and Bool keep their primops, Double keeps row
//! 547's float primops, and a key without such a row keeps today's primop.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use super::DictPassCtxChirho;
use crate::dict_chirho::prelude_chirho::ordering_tests_chirho::ORDERING_TESTS_CHIRHO;
use crate::dict_chirho::type_keys_chirho::canonical_type_key_chirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

/// Keys whose ordering the primops already serve: Int, Char and Bool as Int primops,
/// Double through row 547.
const PRIMOP_KEYS_CHIRHO: [&str; 4] = ["Int", "Char", "Bool", "Double"];

impl DictPassCtxChirho {
    /// `test (compare a b)` for an ordering primop whose operand key has a body-backed
    /// compare row, from the already rewritten operands; `None` keeps the primop.
    pub(super) fn compare_route_chirho(
        &self,
        name_chirho: &str,
        args_chirho: &[CoreExprChirho],
        rewritten_args_chirho: &[CoreExprChirho],
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<CoreExprChirho> {
        let (_, test_name_chirho, _, _) = ORDERING_TESTS_CHIRHO
            .iter()
            .find(|(primop_chirho, ..)| *primop_chirho == name_chirho)?;
        let [left_chirho, right_chirho] = rewritten_args_chirho else {
            return None;
        };
        let key_chirho = canonical_type_key_chirho(&args_chirho.iter().find_map(|arg_chirho| {
            self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                .and_then(Self::concrete_local_type_key_chirho)
        })?);
        if PRIMOP_KEYS_CHIRHO.contains(&key_chirho.as_str()) {
            return None;
        }
        let compare_id_chirho =
            self.lookup_dispatch_body_name_id_chirho(&format!("$prim_Ord_compare_{key_chirho}"))?;
        let test_id_chirho = self.lookup_body_backed_name_id_chirho(test_name_chirho)?;
        let compare_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(compare_id_chirho)),
                arg_chirho: Box::new(left_chirho.clone()),
            }),
            arg_chirho: Box::new(right_chirho.clone()),
        };
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(test_id_chirho)),
            arg_chirho: Box::new(compare_chirho),
        })
    }
}
