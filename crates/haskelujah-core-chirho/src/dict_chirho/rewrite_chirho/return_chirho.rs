// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Apply a result annotation to its own return/pure head, not to its arguments.
//! Workflow: monadic-dispatch-chirho.md.

use super::{CoreExprChirho, DictPassCtxChirho};

impl DictPassCtxChirho {
    pub(super) fn rewrite_proven_return_pure_head_chirho(
        &self,
        expression_chirho: &CoreExprChirho,
        key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        let implementation_chirho = self
            .lookup_dispatch_body_name_id_chirho(&format!("$prim_Applicative_pure_{key_chirho}"))?;
        let mut rewritten_chirho = expression_chirho.clone();
        let mut head_chirho = &mut rewritten_chirho;
        loop {
            match head_chirho {
                CoreExprChirho::AppChirho { fun_chirho, .. } => head_chirho = fun_chirho,
                CoreExprChirho::LamChirho { body_chirho, .. }
                | CoreExprChirho::TyLamChirho { body_chirho, .. }
                | CoreExprChirho::TyAppChirho {
                    expr_chirho: body_chirho,
                    ..
                } => {
                    head_chirho = body_chirho;
                }
                CoreExprChirho::VarChirho(id_chirho)
                    if !self.local_shadow_ids_chirho.borrow().contains(id_chirho)
                        && self.names_chirho.get(id_chirho).is_some_and(|name_chirho| {
                            name_chirho == "return" || name_chirho == "pure"
                        }) =>
                {
                    *id_chirho = implementation_chirho;
                    return Some(rewritten_chirho);
                }
                _ => return None,
            }
        }
    }
}
