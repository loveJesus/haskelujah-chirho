// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A slot of a context instance's own dictionary (`$d<Class>Self`), which the pass builds for
//! the rows of an instance such as `instance Enum a => Enum (WrapChirho a)` (row 671). A slot
//! takes the row bound under the method's name at that key, applied to the context
//! dictionaries, as before: the instance's own row, or a default already made for an earlier
//! slot. An Enum method the instance omits next takes GHC's default, made as a row that leads
//! with the same dictionaries and passes them to the instance's own `toEnum` and `fromEnum`
//! (`context_default_row_chirho`). Only a name nothing can answer gets a stub that fails by
//! the method's name, bound under the id any earlier reference already holds, so the
//! dictionary never carries a dangling id. Nothing generates rows after this pass's per-binding
//! rewrite begins, so no stub can occupy an id that a later default would have supplied.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::DictPassCtxChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

impl DictPassCtxChirho {
    /// The value of `method_name_chirho`'s slot in the own dictionary of `class_name_chirho`'s
    /// context instance at `type_key_chirho`, whose context dictionaries are
    /// `context_dicts_chirho`.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn context_slot_chirho(
        &mut self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        type_key_chirho: &str,
        context_dicts_chirho: &[CoreIdChirho],
    ) -> CoreExprChirho {
        let prim_name_chirho =
            format!("$prim_{class_name_chirho}_{method_name_chirho}_{type_key_chirho}");
        let with_context_chirho = |row_chirho: CoreIdChirho| {
            context_dicts_chirho.iter().fold(
                CoreExprChirho::VarChirho(row_chirho),
                |applied_chirho, dict_id_chirho| CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(applied_chirho),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                },
            )
        };
        let row_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
        if self.is_bound_row_chirho(row_id_chirho) {
            return with_context_chirho(row_id_chirho);
        }
        if class_name_chirho == "Enum"
            && let Some(default_id_chirho) = self.context_default_row_chirho(
                method_name_chirho,
                type_key_chirho,
                context_dicts_chirho.len(),
            )
        {
            return with_context_chirho(default_id_chirho);
        }
        let arity_chirho = self
            .method_arities_chirho
            .get(&(
                class_name_chirho.to_string(),
                method_name_chirho.to_string(),
            ))
            .copied()
            .unwrap_or(0);
        CoreExprChirho::VarChirho(self.missing_method_row_chirho(
            Some(row_id_chirho),
            &prim_name_chirho,
            format!(
                "missing method {class_name_chirho}.{method_name_chirho} for {type_key_chirho}"
            ),
            arity_chirho,
            None,
        ))
    }

    /// Whether a binding of the module or of this pass binds `id_chirho`.
    fn is_bound_row_chirho(&self, id_chirho: CoreIdChirho) -> bool {
        self.module_bound_ids_chirho.contains(&id_chirho)
            || self
                .generated_bindings_chirho
                .iter()
                .any(|binding_chirho| binding_chirho.binder_chirho.id_chirho == id_chirho)
    }
}
