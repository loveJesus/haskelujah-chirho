// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin class kinds from the module's imports (K01 C). Naming proves which class
//! spellings name the compiler's own classes; this child binds their kinds and judges
//! their instance heads.
//! workflow: compiler-pipeline-chirho/builtin-class-identity-chirho

use haskelujah_ast_chirho::name_chirho::{BuiltinClassChirho, BuiltinClassScopeChirho};

use super::sorts_chirho::kind_after_arguments_chirho;
use super::{
    DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO, KindChirho, KindInferCtxChirho,
    SpanChirho, TypeChirho,
};

impl KindInferCtxChirho {
    /// Record the builtin type names, the environment before anything is learned from a
    /// use, and bind each spelling naming proved for a builtin class to that class's kind.
    /// The scope already excludes local and source homonyms; a local declaration is
    /// skipped again here.
    pub(super) fn seed_builtin_kinds_chirho(&mut self, scope_chirho: &BuiltinClassScopeChirho) {
        self.builtin_type_names_chirho = self.env_chirho.kinds_chirho.keys().cloned().collect();
        for (spelling_chirho, class_chirho) in scope_chirho.iter_chirho() {
            if self.local_kind_decl_names_chirho.contains(spelling_chirho) {
                continue;
            }
            self.env_chirho
                .bind_chirho(spelling_chirho.to_string(), class_kind_chirho(class_chirho));
            self.builtin_classes_chirho
                .insert(spelling_chirho.to_string(), *class_chirho);
        }
    }

    /// Whether `spelling_chirho` names a builtin class in this module.
    pub(super) fn is_builtin_class_spelling_chirho(&self, spelling_chirho: &str) -> bool {
        self.builtin_classes_chirho.contains_key(spelling_chirho)
    }

    /// Judge the instance head of a builtin class, whatever the representability switch
    /// says: only a single-parameter class with exactly one written argument, which cannot
    /// be misaligned, and only when that argument's kind is known without inference and
    /// variable-free. Returns whether `spelling_chirho` names a builtin class, so the
    /// local-class path is skipped for it.
    pub(super) fn check_builtin_instance_head_chirho(
        &mut self,
        spelling_chirho: &str,
        head_types_chirho: &[TypeChirho],
        span_chirho: SpanChirho,
    ) -> bool {
        let Some(class_chirho) = self.builtin_classes_chirho.get(spelling_chirho).copied() else {
            return false;
        };
        let ([head_chirho], [arity_chirho]) =
            (head_types_chirho, class_chirho.parameter_arities_chirho)
        else {
            return true;
        };
        let Some(found_chirho) = self.known_head_kind_chirho(head_chirho) else {
            return true;
        };
        if !found_chirho.free_vars_chirho().is_empty() {
            return true;
        }
        let expected_chirho = parameter_kind_chirho(*arity_chirho);
        if found_chirho != expected_chirho {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                    format!(
                        "kind mismatch in the instance head of `{spelling_chirho}`: \
                         expected `{expected_chirho}`, found `{found_chirho}`"
                    ),
                    span_chirho,
                ));
        }
        true
    }

    /// The kind of `ty_chirho` read from its outermost constructor without inference: a
    /// declaration of this module or a builtin type, after its written arguments; a list,
    /// tuple or function type is `*`. `None` for a variable head, an imported constructor
    /// (its kind is learned from a use) or any other form.
    fn known_head_kind_chirho(&self, ty_chirho: &TypeChirho) -> Option<KindChirho> {
        let mut head_chirho = ty_chirho;
        let mut arguments_chirho = 0usize;
        loop {
            match head_chirho {
                TypeChirho::AppChirho { fun_chirho, .. } => {
                    arguments_chirho += 1;
                    head_chirho = fun_chirho;
                }
                TypeChirho::ParenChirho { inner_chirho, .. } => head_chirho = inner_chirho,
                _ => break,
            }
        }
        match head_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.full_name_chirho();
                if !self.local_kind_decl_names_chirho.contains(&text_chirho)
                    && !self.builtin_type_names_chirho.contains(&text_chirho)
                {
                    return None;
                }
                let kind_chirho = self
                    .subst_chirho
                    .apply_chirho(self.env_chirho.lookup_chirho(&text_chirho)?);
                kind_after_arguments_chirho(kind_chirho, arguments_chirho)
            }
            TypeChirho::ListChirho { .. }
            | TypeChirho::TupleChirho { .. }
            | TypeChirho::FunChirho { .. }
                if arguments_chirho == 0 =>
            {
                Some(KindChirho::StarChirho)
            }
            _ => None,
        }
    }
}

/// A builtin class's kind: its parameters' kinds, then `Constraint`.
fn class_kind_chirho(class_chirho: &BuiltinClassChirho) -> KindChirho {
    KindChirho::arrow_n_chirho(
        class_chirho
            .parameter_arities_chirho
            .iter()
            .map(|arity_chirho| parameter_kind_chirho(*arity_chirho)),
        KindChirho::ConstraintChirho,
    )
}

/// `Type` after `arity_chirho` `Type` arguments: `0` is `Type`, `1` is `Type -> Type`.
fn parameter_kind_chirho(arity_chirho: u8) -> KindChirho {
    (0..arity_chirho).fold(KindChirho::StarChirho, |result_chirho, _| {
        KindChirho::arrow_chirho(KindChirho::StarChirho, result_chirho)
    })
}
