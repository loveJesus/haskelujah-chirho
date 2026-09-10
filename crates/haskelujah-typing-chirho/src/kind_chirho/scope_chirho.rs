// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lexical lifetime of explicit kind/type binders.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.

use super::{KindBindingChirho, KindInferCtxChirho, TyVarChirho};

impl KindInferCtxChirho {
    /// Open a binder group, preserving two different meanings of each source name:
    /// `env` holds its inferred kind as a type variable; the cache holds its
    /// identity when used AS a kind (including subsequent binder annotations).
    /// An annotation is converted before its own binder enters scope, but after
    /// the preceding binders. Only named entries are saved: new free names survive
    /// and the work is proportional to this group, not the growing environment.
    pub(super) fn with_kind_binders_chirho<ResultChirho>(
        &mut self,
        vars_chirho: &[TyVarChirho],
        body_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> ResultChirho {
        let mut previous_chirho = Vec::with_capacity(vars_chirho.len());
        for binder_chirho in vars_chirho {
            let kind_chirho = binder_chirho
                .kind_annotation_chirho
                .as_ref()
                .map(|annotation_chirho| self.ast_kind_to_kind_ctx_chirho(annotation_chirho))
                .unwrap_or_else(|| self.fresh_kind_chirho());
            let name_chirho = binder_chirho.text_chirho().to_string();
            let identity_chirho = self.fresh_var_chirho();
            let old_kind_chirho = self.env_chirho.bindings_chirho.insert(
                name_chirho.clone(),
                KindBindingChirho::MonoChirho(kind_chirho),
            );
            let old_identity_chirho = self
                .kind_var_cache_chirho
                .insert(name_chirho.clone(), identity_chirho);
            previous_chirho.push((name_chirho, old_kind_chirho, old_identity_chirho));
        }

        let result_chirho = body_chirho(self);
        for (name_chirho, old_kind_chirho, old_identity_chirho) in previous_chirho.into_iter().rev()
        {
            if let Some(kind_chirho) = old_kind_chirho {
                self.env_chirho
                    .bindings_chirho
                    .insert(name_chirho.clone(), kind_chirho);
            } else {
                self.env_chirho.bindings_chirho.remove(&name_chirho);
            }
            if let Some(identity_chirho) = old_identity_chirho {
                self.kind_var_cache_chirho
                    .insert(name_chirho, identity_chirho);
            } else {
                self.kind_var_cache_chirho.remove(&name_chirho);
            }
        }
        result_chirho
    }
}
