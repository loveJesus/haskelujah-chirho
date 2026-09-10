// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local class-instance kind consumers; workflow: declaration-kinds-chirho.
use super::{
    DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO, KindChirho, KindInferCtxChirho,
    SpanChirho, TypeChirho,
};

impl KindInferCtxChirho {
    /// Number of arguments a kind takes before reaching its result, and whether
    /// that count is final. Only a variable in the RESULT position leaves the
    /// arity open (it may still be instantiated to another arrow); a variable
    /// in an argument position is just an un-annotated parameter — `class C a b`
    /// has arity 2 no matter what kinds `a` and `b` turn out to have.
    pub(super) fn kind_arity_chirho(kind_chirho: &KindChirho) -> (usize, bool) {
        let mut arity_chirho = 0;
        let mut cursor_chirho = kind_chirho;
        loop {
            match cursor_chirho {
                KindChirho::ArrowChirho(_, result_chirho) => {
                    arity_chirho += 1;
                    cursor_chirho = result_chirho;
                }
                KindChirho::VarChirho(_) => return (arity_chirho, false),
                _ => return (arity_chirho, true),
            }
        }
    }

    /// Report an instance head that applies its class to the wrong number of
    /// arguments (`class MonadReader a b` + `instance MonadReader Int`), which
    /// GHC rejects with "Expecting one more argument to ...".
    ///
    /// Deliberately narrow: the class must be declared in THIS module (an
    /// imported class may arrive through a placeholder interface whose kind we
    /// do not really know). The existing syntax-representability boundary is
    /// retained; quantified local kinds are instantiated for each instance.
    pub(super) fn check_instance_head_arity_chirho(
        &mut self,
        class_name_chirho: &str,
        head_types_chirho: &[TypeChirho],
        head_forms_representable_chirho: bool,
        span_chirho: SpanChirho,
    ) {
        let head_arg_count_chirho = head_types_chirho.len();
        // Only classes declared in THIS module: an imported class may arrive
        // through a placeholder interface whose kind we do not really know.
        // The existence of a builtin or placeholder entry alone does not prove
        // an imported class's authoritative parameter contract.
        if !self
            .local_kind_decl_names_chirho
            .contains(class_name_chirho)
        {
            return;
        }
        let Some(class_binding_chirho) = self
            .env_chirho
            .lookup_binding_chirho(class_name_chirho)
            .cloned()
        else {
            return;
        };
        let resolved_kind_chirho = self.instantiate_binding_chirho(&class_binding_chirho);
        let (expected_chirho, spine_known_chirho) = Self::kind_arity_chirho(&resolved_kind_chirho);
        if !spine_known_chirho {
            return;
        }
        if expected_chirho == head_arg_count_chirho {
            if head_forms_representable_chirho {
                self.check_instance_head_arg_kinds_chirho(
                    &resolved_kind_chirho,
                    head_types_chirho,
                    span_chirho,
                );
            }
            return;
        }
        // A head form our lowering drops can only make the head look SHORTER
        // than written, never longer — so over-application stays sound even
        // when some head arguments may be unrepresented.
        if !head_forms_representable_chirho && head_arg_count_chirho < expected_chirho {
            return;
        }
        let message_chirho = if head_arg_count_chirho < expected_chirho {
            format!(
                "expecting {} more argument{} to `{class_name_chirho}` in the instance head",
                expected_chirho - head_arg_count_chirho,
                if expected_chirho - head_arg_count_chirho == 1 {
                    ""
                } else {
                    "s"
                },
            )
        } else {
            format!(
                "`{class_name_chirho}` is applied to {head_arg_count_chirho} arguments, but it takes {expected_chirho}",
            )
        };
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }

    /// A fresh local class contract is unified, not compared by old tree shape.
    /// A scoped journal restores only this instance's bindings; no whole-env copy.
    fn check_instance_head_arg_kinds_chirho(
        &mut self,
        class_kind_chirho: &KindChirho,
        head_types_chirho: &[TypeChirho],
        span_chirho: SpanChirho,
    ) {
        self.env_chirho.begin_scope_chirho();
        let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
        let argument_kinds_chirho: Vec<_> = head_types_chirho
            .iter()
            .map(|argument_chirho| self.infer_type_kind_chirho(argument_chirho))
            .collect();
        self.unify_chirho(
            class_kind_chirho,
            &KindChirho::arrow_n_chirho(argument_kinds_chirho, KindChirho::ConstraintChirho),
            "instance head",
            span_chirho,
        );
        self.kind_var_cache_chirho = outer_names_chirho;
        self.env_chirho.end_scope_chirho();
    }
}
