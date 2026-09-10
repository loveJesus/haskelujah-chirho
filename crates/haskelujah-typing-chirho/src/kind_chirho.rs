// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Kind inference
//!
//! Validates that type constructors are applied at the correct kinds.
//! Haskell has a kind system where `*` (Star) is the kind of types and
//! `k1 -> k2` is the kind of type constructors.
//!
//! Examples:
//! - `Int :: *`
//! - `Maybe :: * -> *`
//! - `Either :: * -> * -> *`
//! - `Functor :: (* -> *) -> Constraint`
//!
//! This module implements kind inference with unification variables,
//! analogous to type inference but at the kind level.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    AstKindChirho, DataKindSigChirho, DeclChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

mod constructors_chirho;
mod conversion_chirho;
#[cfg(test)]
mod declaration_tests_chirho;
mod declarations_chirho;
mod dependencies_chirho;
mod environment_chirho;
mod groups_chirho;
mod instances_chirho;
mod runtime_chirho;
#[cfg(test)]
mod scheme_tests_chirho;
mod schemes_chirho;
mod scope_chirho;
#[cfg(test)]
mod scope_tests_chirho;

#[cfg(test)]
mod term_tests_chirho;
mod terms_chirho;
use terms_chirho::unify_kind_chirho;
pub use terms_chirho::{KindChirho, KindErrorChirho, KindSubstChirho, KindVarChirho};

fn is_builtin_typelit_or_typenat_kind_name_chirho(name_chirho: &str) -> bool {
    matches!(
        name_chirho,
        "Nat"
            | "Symbol"
            | "+"
            | "*"
            | "^"
            | "-"
            | "Div"
            | "Mod"
            | "<=?"
            | "AppendSymbol"
            | "Log2"
            | "CharToNat"
            | "NatToChar"
    )
}

// ---------------------------------------------------------------------------
// Kind environment & inference context
// ---------------------------------------------------------------------------

pub use environment_chirho::KindEnvChirho;
use schemes_chirho::{KindBindingChirho, KindSchemeChirho};

/// Kind inference context with fresh variable generation.
#[derive(Debug, Clone)]
struct KindTypeSynonymChirho {
    params_chirho: Vec<String>,
    rhs_chirho: TypeChirho,
}

struct KindInferCtxChirho {
    env_chirho: KindEnvChirho,
    subst_chirho: KindSubstChirho,
    next_var_chirho: u32,
    diagnostics_chirho: DiagnosticBundleChirho,
    /// Cache for PolyKinds: maps source-level kind variable names to allocated KindVarChirho.
    kind_var_cache_chirho: std::collections::HashMap<String, KindVarChirho>,
    local_kind_decl_names_chirho: std::collections::HashSet<String>,
    local_promoted_constructor_names_chirho: std::collections::HashSet<String>,
    source_kind_qualifiers_chirho: HashMap<String, Option<String>>,
    local_kind_module_chirho: Option<String>,
    type_kind_synonyms_chirho: HashMap<String, KindTypeSynonymChirho>,
    expanding_type_kind_synonyms_chirho: Vec<String>,
    cusks_enabled_chirho: bool,
    poly_kinds_enabled_chirho: bool,
    pending_written_kinds_chirho: Vec<(KindVarChirho, SpanChirho)>,
    /// Scoped source-variable provenance while elaborating an inline kind.
    captured_kind_variables_chirho: Option<Vec<KindVarChirho>>,
}

/// Error codes for kind diagnostics.
const KIND_MISMATCH_CODE_CHIRHO: u16 = 300;
const KIND_OCCURS_CODE_CHIRHO: u16 = 301;

impl KindInferCtxChirho {
    fn new_chirho(env_chirho: KindEnvChirho) -> Self {
        let next_var_chirho = env_chirho
            .bindings_chirho
            .values()
            .flat_map(|binding_chirho| binding_chirho.body_chirho().free_vars_chirho())
            .map(|variable_chirho| variable_chirho.0 + 1)
            .max()
            .unwrap_or(0);
        Self {
            env_chirho,
            subst_chirho: KindSubstChirho::empty_chirho(),
            next_var_chirho,
            diagnostics_chirho: DiagnosticBundleChirho::empty_chirho(),
            kind_var_cache_chirho: std::collections::HashMap::new(),
            local_kind_decl_names_chirho: std::collections::HashSet::new(),
            local_promoted_constructor_names_chirho: std::collections::HashSet::new(),
            source_kind_qualifiers_chirho: HashMap::new(),
            local_kind_module_chirho: None,
            type_kind_synonyms_chirho: HashMap::new(),
            expanding_type_kind_synonyms_chirho: Vec::new(),
            cusks_enabled_chirho: true,
            poly_kinds_enabled_chirho: true,
            pending_written_kinds_chirho: Vec::new(),
            captured_kind_variables_chirho: None,
        }
    }

    fn fresh_var_chirho(&mut self) -> KindVarChirho {
        let v_chirho = KindVarChirho(self.next_var_chirho);
        self.next_var_chirho += 1;
        v_chirho
    }

    fn fresh_kind_chirho(&mut self) -> KindChirho {
        KindChirho::VarChirho(self.fresh_var_chirho())
    }

    fn lookup_type_kind_synonym_chirho(&self, name_chirho: &str) -> Option<&KindTypeSynonymChirho> {
        self.type_kind_synonyms_chirho.get(name_chirho).or_else(|| {
            name_chirho
                .rsplit_once('.')
                .and_then(|(_qualifier_chirho, bare_chirho)| {
                    self.type_kind_synonyms_chirho.get(bare_chirho)
                })
        })
    }

    fn collect_type_app_spine_chirho(
        ty_chirho: &TypeChirho,
        args_chirho: &mut Vec<TypeChirho>,
    ) -> TypeChirho {
        match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                args_chirho.push(arg_chirho.as_ref().clone());
                Self::collect_type_app_spine_chirho(fun_chirho, args_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::collect_type_app_spine_chirho(inner_chirho, args_chirho)
            }
            _ => ty_chirho.clone(),
        }
    }

    fn expand_type_kind_synonym_once_chirho(
        &self,
        ty_chirho: &TypeChirho,
    ) -> Option<(String, TypeChirho)> {
        let mut args_chirho = Vec::new();
        let head_chirho = Self::collect_type_app_spine_chirho(ty_chirho, &mut args_chirho);
        args_chirho.reverse();

        let TypeChirho::ConChirho(name_chirho) = head_chirho else {
            return None;
        };
        let synonym_name_chirho = name_chirho.full_name_chirho();
        let synonym_chirho = self.lookup_type_kind_synonym_chirho(&synonym_name_chirho)?;
        if args_chirho.len() < synonym_chirho.params_chirho.len() {
            return None;
        }

        let mut expanded_chirho = synonym_chirho.rhs_chirho.clone();
        for (param_chirho, arg_chirho) in
            synonym_chirho.params_chirho.iter().zip(args_chirho.iter())
        {
            expanded_chirho = Self::substitute_type_kind_synonym_param_chirho(
                &expanded_chirho,
                param_chirho,
                arg_chirho,
            );
        }
        for arg_chirho in args_chirho.iter().skip(synonym_chirho.params_chirho.len()) {
            expanded_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(expanded_chirho),
                arg_chirho: Box::new(arg_chirho.clone()),
                span_chirho: ty_chirho.span_chirho(),
            };
        }

        if &expanded_chirho == ty_chirho {
            None
        } else {
            Some((synonym_name_chirho, expanded_chirho))
        }
    }

    fn substitute_type_kind_synonym_param_chirho(
        ty_chirho: &TypeChirho,
        param_chirho: &str,
        arg_chirho: &TypeChirho,
    ) -> TypeChirho {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == param_chirho => {
                arg_chirho.clone()
            }
            TypeChirho::VarChirho(_)
            | TypeChirho::ConChirho(_)
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => ty_chirho.clone(),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho: inner_arg_chirho,
                span_chirho,
            } => TypeChirho::AppChirho {
                fun_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    fun_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                arg_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    inner_arg_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::FunChirho {
                arg_chirho: fun_arg_chirho,
                mult_chirho,
                result_chirho,
                span_chirho,
            } => TypeChirho::FunChirho {
                arg_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    fun_arg_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                mult_chirho: mult_chirho.as_ref().map(|multiplicity_chirho| {
                    match multiplicity_chirho {
                        MultiplicityChirho::ExpressionChirho(expression_chirho) => {
                            MultiplicityChirho::ExpressionChirho(Box::new(
                                Self::substitute_type_kind_synonym_param_chirho(
                                    expression_chirho,
                                    param_chirho,
                                    arg_chirho,
                                ),
                            ))
                        }
                        _ => multiplicity_chirho.clone(),
                    }
                }),
                result_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    result_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => TypeChirho::TupleChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        Self::substitute_type_kind_synonym_param_chirho(
                            element_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                span_chirho: *span_chirho,
            },
            TypeChirho::ListChirho {
                element_chirho,
                span_chirho,
            } => TypeChirho::ListChirho {
                element_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    element_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::ParenChirho {
                inner_chirho,
                span_chirho,
            } => TypeChirho::ParenChirho {
                inner_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    inner_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                span_chirho,
            } => TypeChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        Self::substitute_constraint_kind_synonym_param_chirho(
                            constraint_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                body_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    body_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => {
                if vars_chirho
                    .iter()
                    .any(|var_chirho| var_chirho.text_chirho() == param_chirho)
                {
                    ty_chirho.clone()
                } else {
                    TypeChirho::ForallChirho {
                        vars_chirho: vars_chirho.clone(),
                        body_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                            body_chirho,
                            param_chirho,
                            arg_chirho,
                        )),
                        span_chirho: *span_chirho,
                    }
                }
            }
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => {
                if vars_chirho
                    .iter()
                    .any(|var_chirho| var_chirho.text_chirho() == param_chirho)
                {
                    ty_chirho.clone()
                } else {
                    TypeChirho::RequiredForallChirho {
                        vars_chirho: vars_chirho.clone(),
                        body_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                            body_chirho,
                            param_chirho,
                            arg_chirho,
                        )),
                        span_chirho: *span_chirho,
                    }
                }
            }
            TypeChirho::PromotedConChirho { .. } => ty_chirho.clone(),
            TypeChirho::PromotedListChirho {
                elements_chirho,
                span_chirho,
            } => TypeChirho::PromotedListChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        Self::substitute_type_kind_synonym_param_chirho(
                            element_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                span_chirho: *span_chirho,
            },
        }
    }

    fn substitute_constraint_kind_synonym_param_chirho(
        constraint_chirho: &ConstraintChirho,
        param_chirho: &str,
        arg_chirho: &TypeChirho,
    ) -> ConstraintChirho {
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                span_chirho,
            } => ConstraintChirho::ClassChirho {
                class_chirho: class_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|arg_ty_chirho| {
                        Self::substitute_type_kind_synonym_param_chirho(
                            arg_ty_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                span_chirho: *span_chirho,
            },
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                span_chirho,
            } => {
                if vars_chirho
                    .iter()
                    .any(|var_chirho| var_chirho.text_chirho() == param_chirho)
                {
                    constraint_chirho.clone()
                } else {
                    ConstraintChirho::QuantifiedChirho {
                        vars_chirho: vars_chirho.clone(),
                        context_chirho: context_chirho
                            .iter()
                            .map(|inner_chirho| {
                                Self::substitute_constraint_kind_synonym_param_chirho(
                                    inner_chirho,
                                    param_chirho,
                                    arg_chirho,
                                )
                            })
                            .collect(),
                        body_chirho: Box::new(
                            Self::substitute_constraint_kind_synonym_param_chirho(
                                body_chirho,
                                param_chirho,
                                arg_chirho,
                            ),
                        ),
                        span_chirho: *span_chirho,
                    }
                }
            }
        }
    }

    /// Unify two kinds and accumulate the substitution.
    fn unify_chirho(
        &mut self,
        k1_chirho: &KindChirho,
        k2_chirho: &KindChirho,
        context_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        let k1_applied_chirho = self.subst_chirho.apply_chirho(k1_chirho);
        let k2_applied_chirho = self.subst_chirho.apply_chirho(k2_chirho);
        match unify_kind_chirho(
            &k1_applied_chirho,
            &k2_applied_chirho,
            context_chirho,
            span_chirho,
        ) {
            Ok(s_chirho) => {
                self.subst_chirho = s_chirho.compose_chirho(&self.subst_chirho);
            }
            Err(err_chirho) => {
                let (msg_chirho, span_chirho, code_chirho) = match err_chirho {
                    KindErrorChirho::MismatchChirho {
                        expected_chirho,
                        actual_chirho,
                        context_chirho,
                        span_chirho,
                    } => {
                        let msg_chirho = format!(
                            "kind mismatch in {context_chirho}: expected `{expected_chirho}`, found `{actual_chirho}`"
                        );
                        (msg_chirho, span_chirho, KIND_MISMATCH_CODE_CHIRHO)
                    }
                    KindErrorChirho::OccursCheckChirho {
                        var_chirho,
                        kind_chirho,
                        span_chirho,
                    } => {
                        let msg_chirho =
                            format!("infinite kind: `{var_chirho}` occurs in `{kind_chirho}`");
                        (msg_chirho, span_chirho, KIND_OCCURS_CODE_CHIRHO)
                    }
                };
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(code_chirho),
                        msg_chirho,
                        span_chirho,
                    ));
            }
        }
    }

    /// Process a class declaration to determine the kind of the class.
    fn infer_class_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) {
        // Each class type parameter gets a kind: use the annotation if present,
        // otherwise create a fresh kind variable for inference.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                self.ast_kind_to_kind_ctx_chirho(ann_chirho)
            } else {
                self.fresh_kind_chirho()
            };
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        // Class kind: k1 -> k2 -> ... -> Constraint
        let kind_chirho =
            KindChirho::arrow_n_chirho(param_kinds_chirho, KindChirho::ConstraintChirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "class declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Prebind an alias head; its RHS supplies constraints in its inference SCC.
    fn prepare_type_alias_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) {
        // Each type parameter gets a kind: use the annotation if present,
        // otherwise create a fresh kind variable for inference.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                self.ast_kind_to_kind_ctx_chirho(ann_chirho)
            } else {
                self.fresh_kind_chirho()
            };
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        let rhs_kind_chirho = self.fresh_kind_chirho();

        // The alias kind: k_params -> k_rhs
        let kind_chirho = KindChirho::arrow_n_chirho(param_kinds_chirho, rhs_kind_chirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(&existing_chirho, &kind_chirho, "type alias", span_chirho);
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Process a type family declaration to determine the kind of the family.
    fn infer_type_family_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        result_kind_chirho: Option<&TypeChirho>,
        span_chirho: SpanChirho,
        poly_kinds_enabled_chirho: bool,
    ) {
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                self.ast_kind_to_kind_ctx_chirho(ann_chirho)
            } else {
                self.fresh_kind_chirho()
            };
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        let result_kind_chirho = result_kind_chirho
            .map(|kind_ty_chirho| self.type_to_kind_chirho(kind_ty_chirho))
            .unwrap_or_else(|| {
                if poly_kinds_enabled_chirho {
                    self.fresh_kind_chirho()
                } else {
                    KindChirho::StarChirho
                }
            });
        let kind_chirho = KindChirho::arrow_n_chirho(param_kinds_chirho, result_kind_chirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "type family declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Finalize: apply substitution to all kinds in the environment.
    /// When PolyKinds is NOT enabled, default unconstrained kind variables to *.
    /// When PolyKinds IS enabled, preserve kind variables to allow polymorphic kinds.
    fn finalize_chirho(&mut self, _poly_kinds_enabled_chirho: bool) {
        self.env_chirho.apply_subst_chirho(&self.subst_chirho);
        // Quantified variables belong to their scheme, not the ambient
        // substitution/defaulting domain. Only unsolved monomorphic holes default.
        for binding_chirho in self.env_chirho.bindings_chirho.values_mut() {
            binding_chirho.default_unquantified_chirho();
        }
    }
}

/// Convert an AST-level kind annotation to the internal [`KindChirho`] representation.
/// Kind variables (PolyKinds) default to `*` in this standalone version;
/// use `ast_kind_to_kind_ctx_chirho` for proper kind variable allocation.
#[cfg(test)]
fn ast_kind_to_kind_chirho(ast_chirho: &AstKindChirho) -> KindChirho {
    match ast_chirho {
        AstKindChirho::StarChirho => KindChirho::StarChirho,
        AstKindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
            ast_kind_to_kind_chirho(a_chirho),
            ast_kind_to_kind_chirho(b_chirho),
        ),
        AstKindChirho::ConstraintChirho => KindChirho::ConstraintChirho,
        AstKindChirho::ConChirho(name_chirho) => {
            KindChirho::ConChirho(name_chirho.full_name_chirho())
        }
        AstKindChirho::AppChirho(fun_chirho, arg_chirho) => KindChirho::app_chirho(
            ast_kind_to_kind_chirho(fun_chirho),
            ast_kind_to_kind_chirho(arg_chirho),
        ),
        // PolyKinds: kind variables default to * when used outside a context
        AstKindChirho::VarChirho(_) => KindChirho::StarChirho,
    }
}

/// Replace all remaining kind variables with *.
fn default_kind_vars_chirho(kind_chirho: &KindChirho) -> KindChirho {
    kind_chirho.map_leaves_chirho(&mut |term_chirho| match term_chirho {
        KindChirho::VarChirho(_) => KindChirho::StarChirho,
        _ => term_chirho.clone(),
    })
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Result of kind inference on a module.
#[derive(Debug)]
pub struct KindResultChirho {
    /// The kind environment after inference (type constructors → kinds).
    pub env_chirho: KindEnvChirho,
    /// Diagnostics (errors and warnings) from kind checking.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Run kind inference on a module's type declarations and type signatures.
pub fn infer_module_kinds_chirho(module_chirho: &ModuleChirho) -> KindResultChirho {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    ctx_chirho.record_source_kind_qualifiers_chirho(module_chirho);
    ctx_chirho.cusks_enabled_chirho =
        constructors_chirho::cusks_enabled_chirho(&module_chirho.extensions_chirho);
    let poly_kinds_enabled_chirho =
        constructors_chirho::poly_kinds_enabled_chirho(&module_chirho.extensions_chirho);
    ctx_chirho.poly_kinds_enabled_chirho = poly_kinds_enabled_chirho;
    ctx_chirho.check_kind_declarations_chirho(module_chirho);

    // Phase 1.5: instance heads. Runs as its own pass so every class kind is
    // established regardless of declaration order (an instance may precede its
    // class in the source). Instance heads were never kind-checked at all, so
    // `class MonadReader a b` + `instance MonadReader Int` was accepted.
    //
    // Under-application is only reported when every head form is one our
    // lowering represents faithfully. It is NOT, under these extensions: a head
    // argument may be a type-level literal (`instance A 0`), a promoted list
    // (`instance All c '[]`), an operator type (`instance Category (->)`), or a
    // variable-headed application (`instance MonadReader r (Reader r)`, which
    // lowers to ONE argument instead of two). None of those reach
    // `types_chirho`, so the head looks short and we would report OUR gap as
    // the program's error. Over-application stays sound either way — a dropped
    // argument can only shorten the head.
    let instance_head_forms_representable_chirho =
        !module_chirho.extensions_chirho.iter().any(|e_chirho| {
            e_chirho == "DataKinds"
                || e_chirho == "PolyKinds"
                || e_chirho == "TypeInType"
                || e_chirho == "TypeOperators"
                || e_chirho == "FlexibleInstances"
        });
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::InstanceDeclChirho {
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        ctx_chirho.check_instance_head_arity_chirho(
            class_chirho.text_chirho(),
            types_chirho,
            instance_head_forms_representable_chirho,
            *span_chirho,
        );
    }

    // Phase 2: Finalize — apply substitution and default unconstrained vars.
    ctx_chirho.finalize_chirho(poly_kinds_enabled_chirho);

    KindResultChirho {
        env_chirho: ctx_chirho.env_chirho,
        diagnostics_chirho: ctx_chirho.diagnostics_chirho,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::{
        AssocTypeFamilyChirho, AstKindChirho, ClassMethodChirho, ConDeclChirho, DeclChirho,
        StrictnessChirho, TyVarChirho,
    };
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            s_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
        ModuleChirho {
            name_chirho: mk_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho,
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_fun_chirho(arg_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
        TypeChirho::FunChirho {
            arg_chirho: Box::new(arg_chirho),
            mult_chirho: None,
            result_chirho: Box::new(result_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_app_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
        TypeChirho::AppChirho {
            fun_chirho: Box::new(fun_chirho),
            arg_chirho: Box::new(arg_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    // -- Kind representation tests --

    #[test]
    fn star_display_chirho() {
        assert_eq!(KindChirho::StarChirho.to_string(), "*");
    }

    #[test]
    fn arrow_display_chirho() {
        let k_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        assert_eq!(k_chirho.to_string(), "* -> *");
    }

    #[test]
    fn required_forall_kind_keeps_its_visible_parameter_chirho() {
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
        let type_kind_chirho = TypeChirho::ConChirho(mk_name_chirho("Type"));
        let required_forall_chirho = TypeChirho::RequiredForallChirho {
            vars_chirho: vec![TyVarChirho::annotated_chirho(
                mk_name_chirho("kindChirho"),
                AstKindChirho::StarChirho,
            )],
            body_chirho: Box::new(mk_fun_chirho(type_kind_chirho.clone(), type_kind_chirho)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        assert_eq!(
            ctx_chirho.type_to_kind_chirho(&required_forall_chirho),
            KindChirho::arrow_n_chirho(
                [KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho,
            )
        );
    }

    #[test]
    fn arrow_display_nested_chirho() {
        // (* -> *) -> *
        let k_chirho = KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::StarChirho,
        );
        assert_eq!(k_chirho.to_string(), "(* -> *) -> *");
    }

    #[test]
    fn arrow_n_builds_curried_chirho() {
        let k_chirho = KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho,
        );
        assert_eq!(k_chirho.to_string(), "* -> * -> *");
    }

    #[test]
    fn free_vars_chirho() {
        let v_chirho = KindVarChirho(0);
        let k_chirho =
            KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho);
        assert_eq!(k_chirho.free_vars_chirho(), vec![v_chirho]);
    }

    // -- Kind substitution tests --

    #[test]
    fn subst_applies_chirho() {
        let v_chirho = KindVarChirho(0);
        let subst_chirho = KindSubstChirho::singleton_chirho(v_chirho, KindChirho::StarChirho);
        let result_chirho = subst_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho));
        assert_eq!(result_chirho, KindChirho::StarChirho);
    }

    #[test]
    fn subst_compose_chirho() {
        let v0_chirho = KindVarChirho(0);
        let v1_chirho = KindVarChirho(1);
        let s1_chirho =
            KindSubstChirho::singleton_chirho(v0_chirho, KindChirho::VarChirho(v1_chirho));
        let s2_chirho = KindSubstChirho::singleton_chirho(v1_chirho, KindChirho::StarChirho);
        let composed_chirho = s2_chirho.compose_chirho(&s1_chirho);
        // v0 should map to * (through v1)
        assert_eq!(
            composed_chirho.apply_chirho(&KindChirho::VarChirho(v0_chirho)),
            KindChirho::StarChirho
        );
    }

    // -- Kind unification tests --

    #[test]
    fn unify_star_star_chirho() {
        let result_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::StarChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(result_chirho.is_ok());
        assert!(result_chirho.unwrap().is_empty_chirho());
    }

    #[test]
    fn unify_var_with_star_chirho() {
        let v_chirho = KindVarChirho(0);
        let result_chirho = unify_kind_chirho(
            &KindChirho::VarChirho(v_chirho),
            &KindChirho::StarChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
        assert_eq!(
            result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
            KindChirho::StarChirho
        );
    }

    #[test]
    fn unify_arrow_kinds_chirho() {
        let v_chirho = KindVarChirho(0);
        let k1_chirho =
            KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho);
        let k2_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        let result_chirho =
            unify_kind_chirho(&k1_chirho, &k2_chirho, "test", SpanChirho::DUMMY_CHIRHO).unwrap();
        assert_eq!(
            result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
            KindChirho::StarChirho
        );
    }

    #[test]
    fn unify_mismatch_chirho() {
        // Star and Constraint now unify (ConstraintKinds behavior).
        let result_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::ConstraintChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(result_chirho.is_ok());

        // Arrow vs Star is a genuine mismatch.
        let result2_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result2_chirho,
            Err(KindErrorChirho::MismatchChirho { .. })
        ));
    }

    #[test]
    fn occurs_check_chirho() {
        let v_chirho = KindVarChirho(0);
        let result_chirho = unify_kind_chirho(
            &KindChirho::VarChirho(v_chirho),
            &KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho),
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result_chirho,
            Err(KindErrorChirho::OccursCheckChirho { .. })
        ));
    }

    // -- Kind environment tests --

    #[test]
    fn builtins_have_correct_kinds_chirho() {
        let env_chirho = KindEnvChirho::with_builtins_chirho();
        assert_eq!(
            env_chirho.lookup_chirho("Int"),
            Some(&KindChirho::StarChirho)
        );
        assert_eq!(
            env_chirho.lookup_chirho("Int64"),
            Some(&KindChirho::StarChirho)
        );
        assert_eq!(
            env_chirho.lookup_chirho("Word32"),
            Some(&KindChirho::StarChirho)
        );
        assert_eq!(
            env_chirho.lookup_chirho("Word64"),
            Some(&KindChirho::StarChirho)
        );
        assert_eq!(
            env_chirho.lookup_chirho("Maybe"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
        assert_eq!(
            env_chirho.lookup_chirho("Either"),
            Some(&KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho
            ))
        );
        assert_eq!(
            env_chirho.lookup_chirho("GHC.TypeNats.*"),
            Some(&KindChirho::arrow_n_chirho(
                vec![runtime_chirho::builtin_term_chirho("Nat").unwrap(); 2],
                runtime_chirho::builtin_term_chirho("Nat").unwrap()
            )),
            "qualified TypeNats operator lookup should reuse the bare builtin family kind"
        );
        assert_eq!(
            env_chirho.lookup_chirho("AppendSymbol"),
            Some(&KindChirho::arrow_n_chirho(
                vec![runtime_chirho::builtin_term_chirho("Symbol").unwrap(); 2],
                runtime_chirho::builtin_term_chirho("Symbol").unwrap()
            ))
        );
        for (family_chirho, argument_chirho, result_chirho) in [
            ("CharToNat", "Char", "Nat"),
            ("GHC.TypeLits.NatToChar", "Nat", "Char"),
        ] {
            assert_eq!(
                env_chirho.lookup_chirho(family_chirho),
                Some(&KindChirho::arrow_chirho(
                    runtime_chirho::builtin_term_chirho(argument_chirho).unwrap(),
                    runtime_chirho::builtin_term_chirho(result_chirho).unwrap()
                )),
                "{family_chirho} should have a unary TypeLits family kind"
            );
        }
        assert!(
            matches!(
                env_chirho.lookup_chirho(":"),
                Some(KindChirho::ArrowChirho(_, tail_chirho))
                    if matches!(
                        tail_chirho.as_ref(),
                        KindChirho::ArrowChirho(tail_arg_chirho, result_chirho)
                            if tail_arg_chirho == result_chirho
                                && matches!(result_chirho.as_ref(), KindChirho::AppChirho(head_chirho, _)
                                    if matches!(head_chirho.as_ref(), KindChirho::ConChirho(name_chirho) if name_chirho == "[]"))
                    )
            ),
            "promoted list cons preserves the list kind in its tail and result"
        );
    }

    // -- Module-level kind inference tests --

    #[test]
    fn data_return_kind_constraint_is_rejected_chirho() {
        // GHC-55233: `data Foo :: Constraint` — unconditionally rejected, no
        // extension licenses it.
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Foo"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(TypeChirho::ConChirho(
                mk_name_chirho("Constraint"),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        assert!(
            infer_module_kinds_chirho(&module_chirho)
                .diagnostics_chirho
                .has_errors_chirho(),
            "a data declaration returning Constraint must be rejected"
        );
    }

    #[test]
    fn data_binder_kind_constraint_is_accepted_chirho() {
        // `data Foo (_ :: Constraint)` — GHC ACCEPTS this: the Constraint is the
        // BINDER's kind. Before the data-head binder fix this lowered identically to
        // the case above, which is why the check could not be written.
        let mut binder_chirho: TyVarChirho = mk_name_chirho("_").into();
        binder_chirho.kind_annotation_chirho = Some(AstKindChirho::ConstraintChirho);
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Foo"),
            type_vars_chirho: vec![binder_chirho],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        assert!(
            !infer_module_kinds_chirho(&module_chirho)
                .diagnostics_chirho
                .has_errors_chirho(),
            "a BINDER of kind Constraint is legal and must stay accepted"
        );
    }

    #[test]
    fn data_return_kind_constraint_stands_down_when_shadowed_chirho() {
        // A module declaring its own `Constraint` shadows the wired-in one.
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Constraint"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Foo"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(TypeChirho::ConChirho(
                    mk_name_chirho("Constraint"),
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        assert!(
            !infer_module_kinds_chirho(&module_chirho)
                .diagnostics_chirho
                .has_errors_chirho(),
            "a locally declared Constraint shadows the wired-in kind"
        );
    }

    #[test]
    fn data_no_params_has_kind_star_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Color"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Red"),
                fields_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Color"),
            Some(&KindChirho::StarChirho)
        );
    }

    #[test]
    fn data_one_param_has_kind_star_to_star_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Box"),
            type_vars_chirho: vec![mk_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkBox"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Box"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn data_two_params_chirho() {
        // data Pair a b = MkPair a b
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Pair"),
            type_vars_chirho: vec![mk_name_chirho("a").into(), mk_name_chirho("b").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkPair"),
                fields_chirho: vec![
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                    ),
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(mk_name_chirho("b")),
                    ),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Pair"),
            Some(&KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn higher_kinded_type_param_chirho() {
        // data App f a = MkApp (f a)
        // This legacy-edition control checks defaulting, not GHC2021's
        // more general App :: forall k. (k -> Type) -> k -> Type.
        let mut module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("App"),
            type_vars_chirho: vec![mk_name_chirho("f").into(), mk_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkApp"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("f"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        module_chirho.extensions_chirho = vec!["Haskell2010".into()];
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        // App :: (* -> *) -> * -> *
        let expected_chirho = KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("App"),
            Some(&expected_chirho)
        );
    }

    #[test]
    fn qualified_type_constructor_kind_does_not_collide_chirho() {
        let env_chirho = KindEnvChirho::with_builtins_chirho();
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
        ctx_chirho.env_chirho.bind_chirho(
            "Operator".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

        let ty_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(NameChirho::RawChirho(
                            haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                                "N",
                                "Operator",
                                SpanChirho::DUMMY_CHIRHO,
                            ),
                        ))),
                        arg_chirho: Box::new(TypeChirho::ListChirho {
                            element_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("tok"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("st"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(TypeChirho::ConChirho(NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                        "Control.Monad",
                        "Identity",
                        SpanChirho::DUMMY_CHIRHO,
                    ),
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let _kind_chirho = ctx_chirho.infer_type_kind_chirho(&ty_chirho);
        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "qualified imported type constructors should not collide with local unqualified ones: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn rep_kind_accepts_higher_kinded_argument_chirho() {
        let env_chirho = KindEnvChirho::with_builtins_chirho();
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
        ctx_chirho.env_chirho.bind_chirho(
            "QChirho".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

        let ty_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Rep"))),
            arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("QChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let kind_chirho = ctx_chirho.infer_type_kind_chirho(&ty_chirho);
        let normalized_kind_chirho = ctx_chirho.subst_chirho.apply_chirho(&kind_chirho);
        assert_eq!(
            normalized_kind_chirho,
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
        );
        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "Rep should accept higher-kinded arguments without diagnostics: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn class_single_param_chirho() {
        // class Eq a where eq :: a -> a -> Bool
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: mk_name_chirho("Eq"),
            type_vars_chirho: vec![mk_name_chirho("a").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: mk_name_chirho("eq"),
                ty_chirho: mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                        TypeChirho::ConChirho(mk_name_chirho("Bool")),
                    ),
                ),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Eq"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::ConstraintChirho
            ))
        );
    }

    #[test]
    fn class_higher_kinded_param_chirho() {
        // class Functor f where fmap :: (a -> b) -> f a -> f b
        // f :: * -> *, Functor :: (* -> *) -> Constraint
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: mk_name_chirho("Functor"),
            type_vars_chirho: vec![mk_name_chirho("f").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: mk_name_chirho("fmap"),
                ty_chirho: mk_fun_chirho(
                    // (a -> b)
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                        TypeChirho::VarChirho(mk_name_chirho("b")),
                    ),
                    // f a -> f b
                    mk_fun_chirho(
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                        ),
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("b")),
                        ),
                    ),
                ),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        // Functor :: (* -> *) -> Constraint
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Functor"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::ConstraintChirho
            ))
        );
    }

    #[test]
    fn local_tagged_decl_shadows_builtin_tagged_kind_chirho() {
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("SumSize"),
                type_vars_chirho: vec![mk_name_chirho("f").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: mk_name_chirho("sumSize"),
                    ty_chirho: mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("Tagged")),
                        TypeChirho::VarChirho(mk_name_chirho("f")),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::NewtypeDeclChirho {
                name_chirho: mk_name_chirho("Tagged"),
                type_vars_chirho: vec![TyVarChirho::annotated_chirho(
                    mk_name_chirho("s"),
                    AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ),
                )],
                constructor_chirho: ConDeclChirho::RecordChirho {
                    name_chirho: mk_name_chirho("Tagged"),
                    fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        names_chirho: vec![mk_name_chirho("unTagged")],
                        strictness_chirho: StrictnessChirho::LazyChirho,
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "local Tagged should shadow the builtin Tagged kind: {:?}",
            result_chirho.diagnostics_chirho
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("SumSize"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::ConstraintChirho
            ))
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Tagged"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn class_associated_type_family_shadows_builtin_rep_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: mk_name_chirho("Contravariant"),
                args_chirho: vec![TypeChirho::VarChirho(mk_name_chirho("f"))],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            name_chirho: mk_name_chirho("Representable"),
            type_vars_chirho: vec![mk_name_chirho("f").into()],
            methods_chirho: vec![
                ClassMethodChirho {
                    name_chirho: mk_name_chirho("tabulate"),
                    ty_chirho: mk_fun_chirho(
                        mk_fun_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                            mk_app_chirho(
                                TypeChirho::ConChirho(mk_name_chirho("Rep")),
                                TypeChirho::VarChirho(mk_name_chirho("f")),
                            ),
                        ),
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                        ),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                ClassMethodChirho {
                    name_chirho: mk_name_chirho("index"),
                    ty_chirho: mk_fun_chirho(
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                        ),
                        mk_fun_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                            mk_app_chirho(
                                TypeChirho::ConChirho(mk_name_chirho("Rep")),
                                TypeChirho::VarChirho(mk_name_chirho("f")),
                            ),
                        ),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            associated_tfs_chirho: vec![AssocTypeFamilyChirho {
                name_chirho: mk_name_chirho("Rep"),
                type_vars_chirho: vec![mk_name_chirho("f")],
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "associated Rep family should kind-check inside class methods: {:?}",
            result_chirho.diagnostics_chirho
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Rep"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn type_alias_kind_chirho() {
        // type StringPair = (String, String)
        let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("StringPair"),
            type_vars_chirho: vec![],
            rhs_chirho: TypeChirho::TupleChirho {
                elements_chirho: vec![
                    TypeChirho::ConChirho(mk_name_chirho("String")),
                    TypeChirho::ConChirho(mk_name_chirho("String")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("StringPair"),
            Some(&KindChirho::StarChirho)
        );
    }

    #[test]
    fn standalone_kind_signature_expands_local_kind_synonym_application_chirho() {
        let mut module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("Cat"),
                type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("k"))],
                rhs_chirho: mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                        TypeChirho::ConChirho(mk_name_chirho("Type")),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("FreeCat"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("Cat")),
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                    ),
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("Cat")),
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                    ),
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        // The asserted monomorphic kind is the explicit NoPolyKinds contract.
        // Default-edition generality is exercised by the source integration control.
        module_chirho.extensions_chirho = vec!["NoPolyKinds".into()];
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "local kind synonym applications should expand in standalone kind signatures: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("FreeCat"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(
                    KindChirho::StarChirho,
                    KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
                ),
                KindChirho::arrow_chirho(
                    KindChirho::StarChirho,
                    KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
                )
            ))
        );
    }

    #[test]
    fn type_alias_visible_kind_binder_proxy_function_rhs_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("S"),
            type_vars_chirho: vec![
                TyVarChirho::annotated_chirho(mk_name_chirho("k"), AstKindChirho::StarChirho),
                TyVarChirho::annotated_chirho(
                    mk_name_chirho("a"),
                    AstKindChirho::VarChirho("k".to_string()),
                ),
            ],
            rhs_chirho: mk_fun_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                ),
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                ),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "proxy function alias should kind-check: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn newtype_kind_chirho() {
        // newtype Wrapper a = Wrap a
        let module_chirho = mk_module_chirho(vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: mk_name_chirho("Wrapper"),
            type_vars_chirho: vec![mk_name_chirho("a").into()],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Wrap"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Wrapper"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn empty_module_no_errors_chirho() {
        let module_chirho = mk_module_chirho(vec![]);
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn default_unconstrained_vars_chirho() {
        let k_chirho = KindChirho::VarChirho(KindVarChirho(99));
        assert_eq!(default_kind_vars_chirho(&k_chirho), KindChirho::StarChirho);
    }

    #[test]
    fn constraint_kind_display_chirho() {
        assert_eq!(KindChirho::ConstraintChirho.to_string(), "Constraint");
    }

    #[test]
    fn builtin_typeable_accepts_higher_kinded_argument_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("Typeable1Chirho"),
            type_vars_chirho: vec![],
            rhs_chirho: TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Typeable"))),
                arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Maybe"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Typeable should accept higher-kinded arguments: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Typeable1Chirho"),
            Some(&KindChirho::ConstraintChirho)
        );
    }

    #[test]
    fn standalone_forall_kind_signature_preserves_arrow_kind_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("AppChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(TypeChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho::annotated_chirho(
                    mk_name_chirho("fChirho"),
                    AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ),
                )],
                body_chirho: Box::new(mk_fun_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "forall standalone kind signature should kind-check: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("AppChirho"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn type_family_decl_result_kind_guides_later_applications_chirho() {
        let t_var_chirho = TyVarChirho::plain_chirho(mk_name_chirho("t"));
        let mut module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho: mk_name_chirho("TrivialFamily"),
                type_vars_chirho: vec![t_var_chirho.clone()],
                result_kind_chirho: Some(TypeChirho::ConChirho(mk_name_chirho("Type"))),
                equations_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("ProblemTypeChirho"),
                type_vars_chirho: vec![t_var_chirho],
                rhs_chirho: mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("TrivialFamily")),
                        TypeChirho::VarChirho(mk_name_chirho("t")),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        module_chirho
            .extensions_chirho
            .push("PolyKinds".to_string());

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "type family result kind should make later applications kind-check: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn family_result_kind_is_not_an_arbitrary_kind_at_each_use_chirho() {
        let mut module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho: mk_name_chirho("FamilyChirho"),
                type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
                result_kind_chirho: None,
                equations_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("IndexedChirho"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("FamilyChirho")),
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeSigChirho {
                name_chirho: mk_name_chirho("useIndexedChirho"),
                ty_chirho: mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("IndexedChirho")),
                        TypeChirho::ConChirho(mk_name_chirho("Maybe")),
                    ),
                    mk_fun_chirho(
                        mk_app_chirho(
                            TypeChirho::ConChirho(mk_name_chirho("IndexedChirho")),
                            TypeChirho::ConChirho(mk_name_chirho("Int")),
                        ),
                        TypeChirho::ConChirho(mk_name_chirho("Type")),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        module_chirho
            .extensions_chirho
            .push("PolyKinds".to_string());

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        // GHC 9.14.1 rejects both applications: the term Family k is not
        // interchangeable with the kind OF that term. The former assertion
        // expected this invalid program to pass by freshening the entire term.
        let errors_chirho = result_chirho.diagnostics_chirho.diagnostics_chirho();
        assert_eq!(errors_chirho.len(), 2, "{errors_chirho:?}");
        assert!(
            errors_chirho.iter().all(|error_chirho| {
                error_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(300))
                    && error_chirho.message_chirho.contains("FamilyChirho")
                    && error_chirho.message_chirho.contains("type application")
            }),
            "{errors_chirho:?}"
        );
    }

    #[test]
    fn promoted_constructor_does_not_reuse_same_name_type_constructor_kind_chirho() {
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("R"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("PromotedRAppChirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    TypeChirho::PromotedConChirho {
                        name_chirho: mk_name_chirho("R"),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    TypeChirho::ConChirho(mk_name_chirho("Int")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "promoted constructor should not reuse its same-name type constructor's `*` kind: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("R"),
            Some(&KindChirho::StarChirho)
        );
    }

    #[test]
    fn promoted_list_cons_accepts_polykinded_head_chirho() {
        let mut module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("ConsMaybeChirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho(":")),
                    TypeChirho::ConChirho(mk_name_chirho("Maybe")),
                ),
                TypeChirho::PromotedListChirho {
                    elements_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        module_chirho
            .extensions_chirho
            .push("PolyKinds".to_string());

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "promoted list cons should not force its head to kind `*`: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn gadt_return_type_accepts_promoted_list_cons_chirho() {
        let l_chirho = TypeChirho::VarChirho(mk_name_chirho("l"));
        let ls_chirho = TypeChirho::VarChirho(mk_name_chirho("ls"));
        let t_chirho = TypeChirho::VarChirho(mk_name_chirho("t"));
        let promoted_cons_chirho = mk_app_chirho(
            mk_app_chirho(TypeChirho::ConChirho(mk_name_chirho(":")), l_chirho.clone()),
            ls_chirho.clone(),
        );
        let stack_ls_t_chirho = mk_app_chirho(
            mk_app_chirho(TypeChirho::ConChirho(mk_name_chirho("Stack")), ls_chirho),
            t_chirho.clone(),
        );
        let stack_cons_t_chirho = mk_app_chirho(
            mk_app_chirho(
                TypeChirho::ConChirho(mk_name_chirho("Stack")),
                promoted_cons_chirho,
            ),
            t_chirho.clone(),
        );
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Stack"),
            type_vars_chirho: vec![
                TyVarChirho::plain_chirho(mk_name_chirho("lrs")),
                TyVarChirho::annotated_chirho(
                    mk_name_chirho("t"),
                    AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ),
                ),
            ],
            constructors_chirho: vec![ConDeclChirho::GadtChirho {
                name_chirho: mk_name_chirho("SLayer"),
                ty_chirho: mk_fun_chirho(
                    mk_app_chirho(t_chirho, l_chirho),
                    mk_fun_chirho(stack_ls_t_chirho, stack_cons_t_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "GADT return type should accept `Stack (l ': ls) t`: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn builtin_ghc_generics_representations_accept_partial_apps_chirho() {
        let unit_ty_chirho = TypeChirho::TupleChirho {
            elements_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GK1Chirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("K1")),
                        unit_ty_chirho.clone(),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("Int")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GM1Chirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        mk_app_chirho(
                            TypeChirho::ConChirho(mk_name_chirho("M1")),
                            unit_ty_chirho.clone(),
                        ),
                        unit_ty_chirho.clone(),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GSumChirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho(":+:")),
                        TypeChirho::ConChirho(mk_name_chirho("U1")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GProdChirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho(":*:")),
                        TypeChirho::ConChirho(mk_name_chirho("U1")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "GHC.Generics representation constructors should kind-check when partially applied: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        let expected_rep_functor_kind_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GK1Chirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GM1Chirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GSumChirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GProdChirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
    }

    #[test]
    fn ast_kind_constraint_converts_chirho() {
        let ast_chirho = AstKindChirho::ConstraintChirho;
        assert_eq!(
            ast_kind_to_kind_chirho(&ast_chirho),
            KindChirho::ConstraintChirho
        );
    }

    #[test]
    fn ast_kind_constraint_arrow_converts_chirho() {
        let ast_chirho = AstKindChirho::ArrowChirho(
            Box::new(AstKindChirho::StarChirho),
            Box::new(AstKindChirho::ConstraintChirho),
        );
        let expected_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::ConstraintChirho);
        assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), expected_chirho);
    }

    #[test]
    fn ast_kind_var_defaults_to_star_chirho() {
        // PolyKinds: standalone ast_kind_to_kind_chirho defaults kind vars to *
        let ast_chirho = AstKindChirho::VarChirho("k".to_string());
        assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), KindChirho::StarChirho);
    }

    /// `class C a b` plus `instance C <n args>`.
    fn mk_class_and_instance_module_chirho(
        class_param_names_chirho: &[&str],
        instance_head_types_chirho: Vec<TypeChirho>,
    ) -> ModuleChirho {
        mk_module_chirho(vec![
            DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("CChirho"),
                type_vars_chirho: class_param_names_chirho
                    .iter()
                    .map(|p_chirho| mk_name_chirho(p_chirho).into())
                    .collect(),
                methods_chirho: vec![],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                context_chirho: vec![],
                class_chirho: mk_name_chirho("CChirho"),
                types_chirho: instance_head_types_chirho,
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ])
    }

    #[test]
    fn instance_head_under_applied_class_is_rejected_chirho() {
        let module_chirho = mk_class_and_instance_module_chirho(
            &["a", "b"],
            vec![TypeChirho::ConChirho(mk_name_chirho("Int"))],
        );
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "a two-parameter class applied to one argument must be rejected"
        );
    }

    #[test]
    fn instance_head_over_applied_class_is_rejected_chirho() {
        let module_chirho = mk_class_and_instance_module_chirho(
            &["a"],
            vec![
                TypeChirho::ConChirho(mk_name_chirho("Int")),
                TypeChirho::ConChirho(mk_name_chirho("Bool")),
            ],
        );
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "a one-parameter class applied to two arguments must be rejected"
        );
    }

    #[test]
    fn instance_head_matching_arity_is_accepted_chirho() {
        let module_chirho = mk_class_and_instance_module_chirho(
            &["a", "b"],
            vec![
                TypeChirho::ConChirho(mk_name_chirho("Int")),
                TypeChirho::ConChirho(mk_name_chirho("Bool")),
            ],
        );
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "a correctly-saturated instance head must be accepted: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn instance_head_of_undeclared_class_is_not_judged_chirho() {
        // The class is not declared here, so its arity is unknown to us and no
        // verdict may be reached — an imported class could have any arity.
        let module_chirho = mk_module_chirho(vec![DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: mk_name_chirho("SomeImportedClassChirho"),
            types_chirho: vec![TypeChirho::ConChirho(mk_name_chirho("Int"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "an instance of an undeclared class must not be judged: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn kind_arity_is_final_only_when_tail_is_not_a_variable_chirho() {
        // `k -> *` has arity 1 even though the ARGUMENT kind is a variable —
        // only a variable TAIL leaves the arity open.
        let open_tail_chirho = KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::VarChirho(KindVarChirho(0)),
        );
        assert_eq!(
            KindInferCtxChirho::kind_arity_chirho(&open_tail_chirho),
            (1, false)
        );
        let var_arg_chirho = KindChirho::arrow_chirho(
            KindChirho::VarChirho(KindVarChirho(0)),
            KindChirho::ConstraintChirho,
        );
        assert_eq!(
            KindInferCtxChirho::kind_arity_chirho(&var_arg_chirho),
            (1, true)
        );
    }

    #[test]
    fn kind_var_cache_reuses_same_var_chirho() {
        // PolyKinds: context-aware conversion maps same name to same kind var
        let env_chirho = KindEnvChirho::new_chirho();
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
        let k1_chirho =
            ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
        let k2_chirho =
            ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
        assert_eq!(k1_chirho, k2_chirho);
        // Different name gets different var
        let j_chirho =
            ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("j".to_string()));
        assert_ne!(k1_chirho, j_chirho);
    }
}
