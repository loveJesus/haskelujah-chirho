// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lexical lifetime of explicit kind/type binders.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.

use super::{KindBindingChirho, KindInferCtxChirho, TyVarChirho};

impl KindInferCtxChirho {
    /// Implicit names belong to one signature. Outer class-head identities and
    /// their accumulated constraints remain shared. Fresh ids are monotonic;
    /// explicit binders already restore any shadowed cache entry themselves.
    /// Entry is O(1), and cleanup visits this declaration-local name cache, never
    /// clones/scans the module environment or the accumulated substitution.
    pub(super) fn with_signature_kind_scope_chirho<ResultChirho>(
        &mut self,
        body_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> ResultChirho {
        let first_local_identity_chirho = self.next_var_chirho;
        self.env_chirho.begin_scope_chirho();
        let result_chirho = body_chirho(self);
        self.env_chirho.end_scope_chirho();
        self.kind_var_cache_chirho
            .retain(|_name_chirho, identity_chirho| {
                identity_chirho.0 < first_local_identity_chirho
            });
        result_chirho
    }

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
        self.with_kind_binder_scope_chirho(vars_chirho, false, body_chirho)
    }

    /// Checking a written forall cannot solve its bound kind identities from
    /// the body. Conversion of a kind signature instead records quantifiers.
    pub(super) fn with_checked_kind_binders_chirho<ResultChirho>(
        &mut self,
        vars_chirho: &[TyVarChirho],
        body_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> ResultChirho {
        self.with_kind_binder_scope_chirho(vars_chirho, true, body_chirho)
    }

    fn with_kind_binder_scope_chirho<ResultChirho>(
        &mut self,
        vars_chirho: &[TyVarChirho],
        checking_chirho: bool,
        body_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> ResultChirho {
        let mut previous_chirho = Vec::with_capacity(vars_chirho.len());
        for binder_chirho in vars_chirho {
            let kind_chirho = binder_chirho
                .kind_annotation_chirho
                .as_ref()
                .map(|annotation_chirho| self.ast_kind_to_kind_ctx_chirho(annotation_chirho))
                .unwrap_or_else(|| self.fresh_kind_chirho());
            if checking_chirho && binder_chirho.kind_annotation_chirho.is_some() {
                self.rigidify_kind_variables_chirho(kind_chirho.free_vars_chirho());
            }
            let name_chirho = binder_chirho.text_chirho().to_string();
            let identity_chirho = self.fresh_var_chirho();
            self.kind_binder_names_chirho
                .insert(identity_chirho, name_chirho.clone());
            self.kind_binder_classifiers_chirho
                .insert(identity_chirho, kind_chirho.clone());
            self.kind_binder_specificity_chirho
                .insert(identity_chirho, binder_chirho.specificity_chirho);
            if checking_chirho {
                self.rigidify_kind_variables_chirho([identity_chirho]);
            }
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
