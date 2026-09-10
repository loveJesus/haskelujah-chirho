// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source kind interpretation and type-kind inference retain distinct lookup contracts.
//! Binder lifetimes follow language-features-chirho/rank-n-visible-type-application-chirho.

use super::*;

impl KindInferCtxChirho {
    /// Convert AST kind to internal kind, allocating fresh kind vars for PolyKinds.
    /// Same kind variable name maps to the same KindVarChirho within a declaration.
    pub(super) fn ast_kind_to_kind_ctx_chirho(&mut self, ast_chirho: &AstKindChirho) -> KindChirho {
        match ast_chirho {
            AstKindChirho::StarChirho => KindChirho::StarChirho,
            AstKindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
                self.ast_kind_to_kind_ctx_chirho(a_chirho),
                self.ast_kind_to_kind_ctx_chirho(b_chirho),
            ),
            AstKindChirho::ConstraintChirho => KindChirho::ConstraintChirho,
            AstKindChirho::AppChirho(_, _) => KindChirho::StarChirho,
            AstKindChirho::VarChirho(name_chirho) => self.named_kind_variable_chirho(name_chirho),
        }
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
        KindChirho::VarChirho(var_chirho)
    }

    pub(super) fn infer_constraint_kind_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
    ) -> KindChirho {
        match constraint_chirho {
            ConstraintChirho::ClassChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    let _k_chirho = self.infer_type_kind_chirho(arg_chirho);
                }
                KindChirho::ConstraintChirho
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => self.with_kind_binders_chirho(vars_chirho, |ctx_chirho| {
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
        if let Some((name_chirho, expanded_chirho)) =
            self.expand_type_kind_synonym_once_chirho(ty_chirho)
            && !self
                .expanding_type_kind_synonyms_chirho
                .contains(&name_chirho)
        {
            self.expanding_type_kind_synonyms_chirho.push(name_chirho);
            let kind_chirho = self.type_to_kind_chirho(&expanded_chirho);
            self.expanding_type_kind_synonyms_chirho.pop();
            return kind_chirho;
        }

        match ty_chirho {
            TypeChirho::ConChirho(name_chirho)
                if name_chirho.text_chirho() == "Type" || name_chirho.text_chirho() == "*" =>
            {
                KindChirho::StarChirho
            }
            TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Constraint" => {
                KindChirho::ConstraintChirho
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.full_name_chirho();
                // A named kind (e.g. `N` in `N -> Type`): look it up or
                // create a fresh kind variable.
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(&text_chirho) {
                    // DataKinds: if this name has kind * in the env, it's a
                    // data type being used as a kind. At the kind level, data
                    // types are opaque sorts that classify promoted constructors.
                    // Treat them as equivalent to * for kind-checking purposes,
                    // since promoted constructors ultimately have kind *.
                    let k_chirho = k_chirho.clone();
                    match &k_chirho {
                        KindChirho::StarChirho => KindChirho::StarChirho,
                        _ => k_chirho,
                    }
                } else {
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho.bind_chirho(text_chirho, k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => KindChirho::arrow_chirho(
                self.type_to_kind_chirho(arg_chirho),
                self.type_to_kind_chirho(result_chirho),
            ),
            TypeChirho::VarChirho(name_chirho) => {
                self.named_kind_variable_chirho(name_chirho.text_chirho())
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                // Kind application: if fun has kind (a -> r), apply arg of kind a
                // to get result kind r.
                let f_kind_chirho = self.type_to_kind_chirho(fun_chirho);
                let a_kind_chirho = self.type_to_kind_chirho(arg_chirho);
                let result_chirho = self.fresh_kind_chirho();
                let expected_fun_chirho =
                    KindChirho::arrow_chirho(a_kind_chirho, result_chirho.clone());
                self.unify_chirho(
                    &f_kind_chirho,
                    &expected_fun_chirho,
                    "kind application",
                    *span_chirho,
                );
                result_chirho
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => self.type_to_kind_chirho(inner_chirho),
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => self.with_kind_binders_chirho(vars_chirho, |ctx_chirho| {
                ctx_chirho.type_to_kind_chirho(body_chirho)
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
                let text_chirho = name_chirho.full_name_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(&text_chirho) {
                    let k_chirho = k_chirho.clone();
                    // Poly-kinded constructors instantiate at each use site.
                    // This applies to local classes too: sharing `Forall`'s
                    // kind variables across all superclass/signature uses
                    // incorrectly monomorphizes quantified-constraint helpers
                    // such as `ForallF` before later `ForallT` signatures.
                    if !k_chirho.free_vars_chirho().is_empty() {
                        self.instantiate_kind_chirho(&k_chirho)
                    } else {
                        k_chirho
                    }
                } else {
                    // Unknown type constructor — assign a fresh kind variable.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho.bind_chirho(text_chirho, k_chirho.clone());
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
                result_chirho,
                span_chirho,
                ..
            } => {
                let k_a_chirho = self.infer_type_kind_chirho(arg_chirho);
                let k_b_chirho = self.infer_type_kind_chirho(result_chirho);
                // Both sides of -> must be *
                self.unify_chirho(
                    &k_a_chirho,
                    &KindChirho::StarChirho,
                    "function type argument",
                    *span_chirho,
                );
                self.unify_chirho(
                    &k_b_chirho,
                    &KindChirho::StarChirho,
                    "function type result",
                    *span_chirho,
                );
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
            } => self.with_kind_binders_chirho(vars_chirho, |ctx_chirho| {
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
            // DataKinds: promoted constructors ('True, 'Just, 'Proxy, etc.)
            // have kinds determined by their data constructor types. Since we
            // don't track constructor types in the kind env, assign a fresh
            // kind variable so they can unify with whatever context expects.
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(text_chirho) {
                    let k_chirho = k_chirho.clone();
                    if k_chirho == KindChirho::StarChirho {
                        self.fresh_kind_chirho()
                    } else {
                        k_chirho
                    }
                } else {
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_chirho(text_chirho.to_string(), k_chirho.clone());
                    k_chirho
                }
            }
            // DataKinds: promoted list '[a, b] has kind [*] which we represent as *.
            TypeChirho::PromotedListChirho {
                elements_chirho,
                span_chirho,
            } => {
                for elem_chirho in elements_chirho {
                    let k_chirho = self.infer_type_kind_chirho(elem_chirho);
                    self.unify_chirho(
                        &k_chirho,
                        &KindChirho::StarChirho,
                        "promoted list element",
                        *span_chirho,
                    );
                }
                KindChirho::StarChirho
            }
            // PartialTypeSignatures: `_` is a wildcard that will be filled in
            // during type inference. Kind-wise it is treated as * (a regular
            // monotype position).
            TypeChirho::WildcardChirho { .. } => KindChirho::StarChirho,
            // Type-level literal (DataKinds): literals have kind *.
            TypeChirho::LitChirho { .. } => KindChirho::StarChirho,
        }
    }
}
