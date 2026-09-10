// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Compose result-kind tails and reconcile complete data/newtype signatures.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    DataKindSigChirho, DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO, KindChirho,
    KindInferCtxChirho, SpanChirho, TyVarChirho, TypeChirho,
};

impl KindInferCtxChirho {
    pub(super) fn infer_data_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        kind_sig_chirho: Option<&DataKindSigChirho>,
        span_chirho: SpanChirho,
    ) {
        let standalone_chirho = kind_sig_chirho.and_then(DataKindSigChirho::standalone_chirho);
        let result_chirho = kind_sig_chirho.and_then(DataKindSigChirho::result_chirho);
        // Ordinary unification permits Type/Constraint compatibility, so retain
        // GHC-55233 explicitly on either written contract (one diagnostic).
        if let Some(invalid_chirho) = standalone_chirho
            .into_iter()
            .chain(result_chirho)
            .find(|sig_chirho| self.declared_return_kind_is_constraint_chirho(sig_chirho))
        {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                    format!("data type `{name_chirho}` has non-`*` return kind `Constraint`"),
                    invalid_chirho.span_chirho(),
                ));
        }

        // Standalone binders (including implicitly quantified free kind names)
        // do not scope over the declaration. Swap only the name cache, O(1),
        // retaining fresh ids and substitutions; never clone the growing env.
        let complete_kind_chirho = standalone_chirho.map(|signature_chirho| {
            let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
            let kind_chirho = self.type_to_kind_chirho(signature_chirho);
            self.kind_var_cache_chirho = outer_names_chirho;
            kind_chirho
        });

        let mut parameter_kinds_chirho = Vec::with_capacity(type_vars_chirho.len());
        for variable_chirho in type_vars_chirho {
            let kind_chirho = variable_chirho
                .kind_annotation_chirho
                .as_ref()
                .map(|annotation_chirho| self.ast_kind_to_kind_ctx_chirho(annotation_chirho))
                .unwrap_or_else(|| self.fresh_kind_chirho());
            self.env_chirho.bind_chirho(
                variable_chirho.text_chirho().to_owned(),
                kind_chirho.clone(),
            );
            parameter_kinds_chirho.push(kind_chirho);
        }
        // Head annotations and the inline tail share identities and remain in
        // scope while the caller checks constructor fields, as ordinary heads do.
        let result_kind_chirho = result_chirho
            .map(|tail_chirho| self.type_to_kind_chirho(tail_chirho))
            .unwrap_or(KindChirho::StarChirho);
        let head_kind_chirho =
            KindChirho::arrow_n_chirho(parameter_kinds_chirho, result_kind_chirho);
        let kind_chirho = if let Some(complete_kind_chirho) = complete_kind_chirho {
            self.unify_chirho(
                &complete_kind_chirho,
                &head_kind_chirho,
                "data declaration signature",
                span_chirho,
            );
            complete_kind_chirho
        } else {
            head_kind_chirho
        };
        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho).cloned() {
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "data declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_owned(), kind_chirho);
    }

    /// GHC-55233: the return kind of data/newtype cannot be Constraint. This
    /// differs from a binder's kind; the parser now represents that distinction.
    /// A locally declared Constraint retains the previous shadowing boundary.
    fn declared_return_kind_is_constraint_chirho(&self, sig_chirho: &TypeChirho) -> bool {
        if self.local_kind_decl_names_chirho.contains("Constraint") {
            return false;
        }
        let mut tail_chirho = sig_chirho;
        loop {
            match tail_chirho {
                TypeChirho::FunChirho { result_chirho, .. } => tail_chirho = result_chirho,
                TypeChirho::ParenChirho { inner_chirho, .. } => tail_chirho = inner_chirho,
                TypeChirho::ForallChirho { body_chirho, .. } => tail_chirho = body_chirho,
                _ => break,
            }
        }
        matches!(tail_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Constraint")
    }
}
