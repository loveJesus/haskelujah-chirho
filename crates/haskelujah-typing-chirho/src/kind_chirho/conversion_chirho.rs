// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source kind interpretation and type-kind inference retain distinct lookup contracts.
//! Binder lifetimes follow language-features-chirho/rank-n-visible-type-application-chirho.

use super::*;

impl KindInferCtxChirho {
    /// Convert AST kind to internal kind, allocating fresh kind vars for PolyKinds.
    /// Same kind variable name maps to the same KindVarChirho within a declaration.
    pub(super) fn ast_kind_to_kind_ctx_chirho(&mut self, ast_chirho: &AstKindChirho) -> KindChirho {
        let ty_chirho = ast_kind_type_chirho(ast_chirho);
        self.type_to_kind_chirho(&ty_chirho)
    }

    fn named_kind_variable_chirho(&mut self, name_chirho: &str) -> KindChirho {
        let var_chirho = if let Some(&var_chirho) = self.kind_var_cache_chirho.get(name_chirho) {
            var_chirho
        } else {
            let var_chirho = self.fresh_var_chirho();
            self.kind_var_cache_chirho
                .insert(name_chirho.to_string(), var_chirho);
            var_chirho
        };
        if let Some(captured_chirho) = &mut self.captured_kind_variables_chirho {
            captured_chirho.push(var_chirho);
        }
        KindChirho::VarChirho(var_chirho)
    }

    /// Preserve the origin of a written contract. Fresh application-result
    /// variables and unknown constructor kinds are inference machinery, not
    /// implicitly quantified source names. Only source identities still
    /// unresolved after the annotation itself is elaborated constrain later
    /// constructor-body inference. Work is local to this signature's syntax.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn elaborate_inline_kind_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
    ) -> (KindChirho, Vec<KindVarChirho>) {
        let outer_capture_chirho = self.captured_kind_variables_chirho.replace(Vec::new());
        let kind_chirho = self.type_to_kind_chirho(ty_chirho);
        let captured_chirho = self.captured_kind_variables_chirho.take().unwrap();
        self.captured_kind_variables_chirho = outer_capture_chirho;
        let written_chirho = captured_chirho
            .into_iter()
            .filter_map(|variable_chirho| {
                match self
                    .subst_chirho
                    .apply_chirho(&KindChirho::VarChirho(variable_chirho))
                {
                    KindChirho::VarChirho(unresolved_chirho) => Some(unresolved_chirho),
                    _ => None,
                }
            })
            .collect();
        (kind_chirho, written_chirho)
    }

    pub(super) fn infer_constraint_kind_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
    ) -> KindChirho {
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                span_chirho,
            } => {
                // A superclass is an application just like a type constructor.
                // Ignoring its head loses the only constraint on `m` in
                // `class Monad m => C m`, before C's kind is generalized.
                // A variable predicate head uses its shared Mono binding; a
                // published class scheme is instantiated for this occurrence.
                let binding_chirho = self
                    .env_chirho
                    .lookup_binding_chirho(&class_chirho.full_name_chirho())
                    .cloned();
                let head_chirho = if class_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .is_some_and(|first_chirho| first_chirho.is_lowercase() || first_chirho == '_')
                {
                    // An implicit predicate variable, as in `MkDict :: c =>
                    // Dict c`, must bind c before its later use in Dict c.
                    // Unlike a missing imported class, its kind is ours to infer.
                    Some(self.infer_type_kind_chirho(&TypeChirho::VarChirho(class_chirho.clone())))
                } else {
                    binding_chirho
                        .as_ref()
                        .map(|binding_chirho| self.instantiate_binding_chirho(binding_chirho))
                };
                let argument_kinds_chirho: Vec<_> = args_chirho
                    .iter()
                    .map(|argument_chirho| self.infer_type_kind_chirho(argument_chirho))
                    .collect();
                if let Some(head_chirho) = head_chirho {
                    self.unify_chirho(
                        &head_chirho,
                        &KindChirho::arrow_n_chirho(
                            argument_kinds_chirho,
                            KindChirho::ConstraintChirho,
                        ),
                        "constraint application",
                        *span_chirho,
                    );
                }
                // An imported class without kind metadata still cannot supply
                // an authoritative contract. Its arguments were checked above.
                KindChirho::ConstraintChirho
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => self.with_checked_kind_binders_chirho(vars_chirho, |ctx_chirho| {
                for inner_constraint_chirho in context_chirho {
                    let _k_chirho =
                        ctx_chirho.infer_constraint_kind_chirho(inner_constraint_chirho);
                }
                let _k_chirho = ctx_chirho.infer_constraint_kind_chirho(body_chirho);
                KindChirho::ConstraintChirho
            }),
        }
    }

    /// Interpret a `TypeChirho` as a kind (for standalone kind signatures like
    /// `data V :: N -> Type where`).  This converts the *type-level*
    /// representation of a kind back into a `KindChirho`.
    pub(super) fn type_to_kind_chirho(&mut self, ty_chirho: &TypeChirho) -> KindChirho {
        // Validate applications against constructor kinds, then interpret their
        // TERMS. Keeping the two outputs separate avoids turning TYPE Bool into
        // a plausible runtime kind simply because it has application syntax.
        self.env_chirho.begin_scope_chirho();
        let _classifier_chirho = self.infer_type_kind_chirho(ty_chirho);
        let term_chirho = self.interpret_kind_term_chirho(ty_chirho);
        self.env_chirho.end_scope_chirho();
        term_chirho
    }

    fn interpret_kind_term_chirho(&mut self, ty_chirho: &TypeChirho) -> KindChirho {
        if let Some((name_chirho, expanded_chirho)) =
            self.expand_type_kind_synonym_once_chirho(ty_chirho)
            && !self
                .expanding_type_kind_synonyms_chirho
                .contains(&name_chirho)
        {
            self.expanding_type_kind_synonyms_chirho.push(name_chirho);
            let kind_chirho = self.interpret_kind_term_chirho(&expanded_chirho);
            self.expanding_type_kind_synonyms_chirho.pop();
            return kind_chirho;
        }

        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => self.named_kind_term_chirho(name_chirho),
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                self.named_promoted_kind_term_chirho(name_chirho)
            }
            TypeChirho::LitChirho { value_chirho, .. } => {
                KindChirho::ConChirho(value_chirho.clone())
            }
            TypeChirho::ListChirho { element_chirho, .. } => KindChirho::app_chirho(
                KindChirho::ConChirho("[]".into()),
                self.interpret_kind_term_chirho(element_chirho),
            ),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => KindChirho::arrow_chirho(
                self.interpret_kind_term_chirho(arg_chirho),
                self.interpret_kind_term_chirho(result_chirho),
            ),
            TypeChirho::VarChirho(name_chirho) => {
                self.named_kind_variable_chirho(name_chirho.text_chirho())
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                // This is the term `f a`, not the result kind of applying f.
                KindChirho::app_chirho(
                    self.interpret_kind_term_chirho(fun_chirho),
                    self.interpret_kind_term_chirho(arg_chirho),
                )
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.interpret_kind_term_chirho(inner_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => self.with_kind_binders_chirho(vars_chirho, |ctx_chirho| {
                ctx_chirho.interpret_kind_term_chirho(body_chirho)
            }),
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => self.with_kind_binders_chirho(vars_chirho, |ctx_chirho| {
                let mut result_chirho = ctx_chirho.interpret_kind_term_chirho(body_chirho);
                for binder_chirho in vars_chirho.iter().rev() {
                    let identity_chirho =
                        ctx_chirho.kind_var_cache_chirho[binder_chirho.text_chirho()];
                    let argument_chirho = ctx_chirho
                        .env_chirho
                        .lookup_chirho(binder_chirho.text_chirho())
                        .expect("required kind binder was opened")
                        .clone();
                    if result_chirho.free_vars_chirho().contains(&identity_chirho) {
                        let bound_result_chirho =
                            result_chirho.abstract_variable_chirho(identity_chirho);
                        result_chirho = KindChirho::DependentChirho {
                            argument_chirho: Box::new(argument_chirho),
                            result_chirho: Box::new(bound_result_chirho),
                        };
                    } else {
                        result_chirho = KindChirho::arrow_chirho(argument_chirho, result_chirho);
                    }
                }
                result_chirho
            }),
            _ => {
                // Fallback: treat unknown shapes as *.
                KindChirho::StarChirho
            }
        }
    }

    /// Infer the kind of a type expression.
    pub(super) fn infer_type_kind_chirho(&mut self, ty_chirho: &TypeChirho) -> KindChirho {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(text_chirho) {
                    k_chirho.clone()
                } else {
                    // Unknown type variable — assign a fresh kind variable.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_chirho(text_chirho.to_string(), k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = self.canonical_kind_name_chirho(name_chirho);
                if let Some(binding_chirho) = self
                    .env_chirho
                    .lookup_binding_chirho(&text_chirho)
                    .or_else(|| self.env_chirho.lookup_promoted_binding_chirho(&text_chirho))
                    .cloned()
                {
                    self.instantiate_binding_chirho(&binding_chirho)
                } else {
                    // Unknown/imported constructors still lack authoritative kind
                    // metadata. Keep their existing independent-use fallback,
                    // explicitly separate from local monomorphic recursion.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_generalized_chirho(text_chirho, k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let k_fun_chirho = self.infer_type_kind_chirho(fun_chirho);
                let k_arg_chirho = self.infer_type_kind_chirho(arg_chirho);
                if let KindChirho::DependentChirho {
                    argument_chirho,
                    result_chirho,
                } = self.subst_chirho.apply_chirho(&k_fun_chirho)
                {
                    self.unify_chirho(
                        &argument_chirho,
                        &k_arg_chirho,
                        "dependent type application",
                        *span_chirho,
                    );
                    // Its classifier was checked above. Rechecking the whole
                    // argument here duplicates work at every nested Pi call.
                    let argument_term_chirho = self.interpret_kind_term_chirho(arg_chirho);
                    return self.subst_chirho.apply_chirho(
                        &result_chirho.substitute_bound_chirho(&argument_term_chirho),
                    );
                }
                let k_result_chirho = self.fresh_kind_chirho();
                // fun must have kind (k_arg -> k_result)
                let expected_chirho =
                    KindChirho::arrow_chirho(k_arg_chirho, k_result_chirho.clone());
                self.unify_chirho(
                    &k_fun_chirho,
                    &expected_chirho,
                    "type application",
                    *span_chirho,
                );
                k_result_chirho
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                span_chirho,
            } => {
                let k_a_chirho = self.infer_type_kind_chirho(arg_chirho);
                let k_b_chirho = self.infer_type_kind_chirho(result_chirho);
                if let Some(MultiplicityChirho::ExpressionChirho(expression_chirho)) = mult_chirho {
                    let classifier_chirho = self.infer_type_kind_chirho(expression_chirho);
                    self.unify_chirho(
                        &classifier_chirho,
                        &KindChirho::ConChirho("GHC.Types.Multiplicity".into()),
                        "arrow multiplicity",
                        expression_chirho.span_chirho(),
                    );
                }
                self.check_runtime_kind_chirho(&k_a_chirho, "function type argument", *span_chirho);
                self.check_runtime_kind_chirho(&k_b_chirho, "function type result", *span_chirho);
                KindChirho::StarChirho
            }
            TypeChirho::ListChirho {
                element_chirho,
                span_chirho,
            } => {
                let k_elem_chirho = self.infer_type_kind_chirho(element_chirho);
                self.unify_chirho(
                    &k_elem_chirho,
                    &KindChirho::StarChirho,
                    "list element type",
                    *span_chirho,
                );
                KindChirho::StarChirho
            }
            TypeChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => {
                // Tuple elements can all be * (value tuple) or all Constraint
                // (constraint tuple, e.g. (Show a, Eq a)). Use a fresh kind
                // variable and unify each element against it so the tuple's
                // kind is determined by its contents.
                let elem_kind_chirho = self.fresh_kind_chirho();
                for elem_chirho in elements_chirho {
                    let k_chirho = self.infer_type_kind_chirho(elem_chirho);
                    self.unify_chirho(
                        &k_chirho,
                        &elem_kind_chirho,
                        "tuple element type",
                        *span_chirho,
                    );
                }
                // Resolve the element kind — defaults to * if unconstrained.
                let resolved_chirho = self.subst_chirho.apply_chirho(&elem_kind_chirho);
                match &resolved_chirho {
                    KindChirho::ConstraintChirho => KindChirho::ConstraintChirho,
                    _ => KindChirho::StarChirho,
                }
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.infer_type_kind_chirho(inner_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho: _,
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho: _,
            } => self.with_checked_kind_binders_chirho(vars_chirho, |ctx_chirho| {
                // A forall type has the same kind as its body:
                // - forall a. a -> a  has kind * (body is *)
                // - forall a. C a => D (f a)  has kind Constraint (quantified constraint)
                ctx_chirho.infer_type_kind_chirho(body_chirho)
            }),
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                span_chirho: _,
            } => {
                // Kind-check each constraint in the context.
                // NOTE: We do NOT force constraint arguments to kind *.
                // Constraint arguments like `f` in `Functor f` have kind `* -> *`.
                // We just infer their kinds and let unification propagate.
                for constraint_chirho in context_chirho {
                    let _k_chirho = self.infer_constraint_kind_chirho(constraint_chirho);
                }
                // The body determines the kind of the qualified type:
                // - Num a => a -> a  has kind * (normal qualified type)
                // - NFData a => NFData (f a)  has kind Constraint (quantified constraint)
                self.infer_type_kind_chirho(body_chirho)
            }
            // Known promoted-constructor contracts instantiate per occurrence.
            // Missing constructor metadata remains an opaque-use boundary,
            // not a monomorphic binding and not a same-spelled type constructor.
            // This does not reconstruct existential annotations dropped by AST lowering.
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                if let Some(binding_chirho) = self
                    .env_chirho
                    .lookup_promoted_binding_chirho(&self.canonical_kind_name_chirho(name_chirho))
                    .cloned()
                {
                    self.instantiate_binding_chirho(&binding_chirho)
                } else {
                    self.fresh_kind_chirho()
                }
            }
            // Promotion preserves the common element kind, including Nat and
            // Symbol. A promoted list is not itself a lifted value type.
            TypeChirho::PromotedListChirho {
                elements_chirho,
                span_chirho,
            } => {
                let element_kind_chirho = self.fresh_kind_chirho();
                for elem_chirho in elements_chirho {
                    let k_chirho = self.infer_type_kind_chirho(elem_chirho);
                    self.unify_chirho(
                        &k_chirho,
                        &element_kind_chirho,
                        "promoted list element",
                        *span_chirho,
                    );
                }
                KindChirho::app_chirho(
                    KindChirho::ConChirho("[]".into()),
                    self.subst_chirho.apply_chirho(&element_kind_chirho),
                )
            }
            // PartialTypeSignatures: `_` is a wildcard that will be filled in
            // during type inference. Kind-wise it is treated as * (a regular
            // monotype position).
            TypeChirho::WildcardChirho { .. } => KindChirho::StarChirho,
            TypeChirho::LitChirho { value_chirho, .. } => {
                let name_chirho = if value_chirho.starts_with('"') {
                    "Symbol"
                } else if value_chirho.starts_with('\'') {
                    "Char"
                } else {
                    "Nat"
                };
                super::runtime_chirho::builtin_term_chirho(name_chirho)
                    .expect("literal kinds are builtin nominal terms")
            }
        }
    }
}

fn ast_kind_type_chirho(kind_chirho: &AstKindChirho) -> TypeChirho {
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    let name_chirho = |text_chirho: &str| {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    };
    match kind_chirho {
        AstKindChirho::ConChirho(constructor_chirho) => {
            TypeChirho::ConChirho(constructor_chirho.clone())
        }
        AstKindChirho::StarChirho => TypeChirho::ConChirho(name_chirho("Type")),
        AstKindChirho::ConstraintChirho => TypeChirho::ConChirho(name_chirho("Constraint")),
        AstKindChirho::VarChirho(variable_chirho) => {
            TypeChirho::VarChirho(name_chirho(variable_chirho))
        }
        AstKindChirho::ArrowChirho(left_chirho, right_chirho)
        | AstKindChirho::AppChirho(left_chirho, right_chirho) => {
            let left_chirho = ast_kind_type_chirho(left_chirho);
            let right_chirho = ast_kind_type_chirho(right_chirho);
            let span_chirho = left_chirho
                .span_chirho()
                .merge_chirho(right_chirho.span_chirho())
                .unwrap_or_else(|| {
                    if left_chirho.span_chirho() == SpanChirho::DUMMY_CHIRHO {
                        right_chirho.span_chirho()
                    } else {
                        left_chirho.span_chirho()
                    }
                });
            if matches!(kind_chirho, AstKindChirho::ArrowChirho(_, _)) {
                TypeChirho::FunChirho {
                    arg_chirho: Box::new(left_chirho),
                    result_chirho: Box::new(right_chirho),
                    mult_chirho: None,
                    span_chirho,
                }
            } else {
                TypeChirho::AppChirho {
                    fun_chirho: Box::new(left_chirho),
                    arg_chirho: Box::new(right_chirho),
                    span_chirho,
                }
            }
        }
    }
}
