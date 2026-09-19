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
    AstKindChirho, DeclChirho, DeclKindSigChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

mod constructors_chirho;
mod conversion_chirho;
#[cfg(test)]
#[path = "kind_chirho/tests_chirho/declaration_tests_chirho.rs"]
mod declaration_tests_chirho;
mod declarations_chirho;
#[cfg(test)]
#[path = "kind_chirho/tests_chirho/default_tests_chirho.rs"]
mod default_tests_chirho;
mod defaults_chirho;
mod dependencies_chirho;
#[path = "kind_chirho/bindings_chirho/environment_chirho.rs"]
mod environment_chirho;
mod families_chirho;
#[cfg(test)]
#[path = "kind_chirho/tests_chirho/family_tests_chirho.rs"]
mod family_tests_chirho;
mod family_unify_chirho;
mod groups_chirho;
#[path = "kind_chirho/bindings_chirho/imports_chirho.rs"]
mod imports_chirho;
mod instances_chirho;
pub use imports_chirho::KindContractChirho;
#[cfg(test)]
#[path = "kind_chirho/tests_chirho/import_tests_chirho.rs"]
mod import_tests_chirho;
mod runtime_chirho;
pub(crate) use runtime_chirho::is_builtin_nominal_kind_name_chirho;
#[cfg(test)]
#[path = "kind_chirho/tests_chirho/scheme_tests_chirho.rs"]
mod scheme_tests_chirho;
#[path = "kind_chirho/bindings_chirho/schemes_chirho.rs"]
mod schemes_chirho;
#[path = "kind_chirho/bindings_chirho/scope_chirho.rs"]
mod scope_chirho;
#[cfg(test)]
#[path = "kind_chirho/tests_chirho/scope_tests_chirho.rs"]
mod scope_tests_chirho;

#[cfg(test)]
#[path = "kind_chirho/tests_chirho/term_tests_chirho.rs"]
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
mod elaboration_chirho;
pub(crate) use elaboration_chirho::ClosedFamilyInjectivityChirho;
pub use elaboration_chirho::KindElaborationChirho;
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
    imported_kind_shapes_chirho: HashMap<String, imports_chirho::KindHeadShapeChirho>,
    local_promoted_constructor_names_chirho: std::collections::HashSet<String>,
    source_kind_qualifiers_chirho: HashMap<String, Option<String>>,
    source_builtin_kind_aliases_chirho: HashMap<String, KindChirho>,
    local_kind_module_chirho: Option<String>,
    type_kind_synonyms_chirho: HashMap<String, KindTypeSynonymChirho>,
    expanding_type_kind_synonyms_chirho: Vec<String>,
    cusks_enabled_chirho: bool,
    poly_kinds_enabled_chirho: bool,
    star_is_type_chirho: bool,
    pending_written_kinds_chirho: Vec<(KindVarChirho, SpanChirho)>,
    /// Scoped source-variable provenance while elaborating an inline kind.
    captured_kind_variables_chirho: Option<Vec<KindVarChirho>>,
    /// Scoped instance-header policy, restored before checking its equations.
    rigid_ascription_names_chirho: bool,
    /// Classifiers and specificity belong to the quantified identity, not its
    /// temporary source spelling. They survive lexical-scope restoration.
    kind_binder_classifiers_chirho: HashMap<KindVarChirho, KindChirho>,
    classifier_session_chirho: Option<runtime_chirho::ClassifierSessionChirho>,
    kind_binder_names_chirho: HashMap<KindVarChirho, String>,
    kind_applications_chirho: HashMap<SpanChirho, elaboration_chirho::PendingKindApplicationChirho>,
    kind_class_instances_chirho: HashMap<usize, elaboration_chirho::PendingKindApplicationChirho>,
    kind_wildcard_terms_chirho: HashMap<SpanChirho, KindChirho>,
    kind_equation_inputs_chirho:
        HashMap<SpanChirho, elaboration_chirho::PendingKindApplicationChirho>,
    kind_associated_defaults_chirho: HashMap<(SpanChirho, String), Vec<KindChirho>>,
    kind_binder_specificity_chirho:
        HashMap<KindVarChirho, haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho>,
    kind_families_chirho: HashMap<String, families_chirho::KindFamilyChirho>,
    kind_family_names_chirho: std::collections::HashSet<String>,
    /// Fresh choices at a polymorphic occurrence, not written binder identities.
    instantiated_kind_variables_chirho: std::collections::HashSet<KindVarChirho>,
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
            imported_kind_shapes_chirho: HashMap::new(),
            local_promoted_constructor_names_chirho: std::collections::HashSet::new(),
            source_kind_qualifiers_chirho: HashMap::new(),
            source_builtin_kind_aliases_chirho: HashMap::new(),
            local_kind_module_chirho: None,
            type_kind_synonyms_chirho: HashMap::new(),
            expanding_type_kind_synonyms_chirho: Vec::new(),
            cusks_enabled_chirho: true,
            poly_kinds_enabled_chirho: true,
            star_is_type_chirho: true,
            pending_written_kinds_chirho: Vec::new(),
            captured_kind_variables_chirho: None,
            rigid_ascription_names_chirho: false,
            kind_binder_classifiers_chirho: HashMap::new(),
            classifier_session_chirho: None,
            kind_binder_names_chirho: HashMap::new(),
            kind_applications_chirho: HashMap::new(),
            kind_class_instances_chirho: HashMap::new(),
            kind_wildcard_terms_chirho: HashMap::new(),
            kind_equation_inputs_chirho: HashMap::new(),
            kind_associated_defaults_chirho: HashMap::new(),
            kind_binder_specificity_chirho: HashMap::new(),
            kind_families_chirho: HashMap::new(),
            kind_family_names_chirho: std::collections::HashSet::new(),
            instantiated_kind_variables_chirho: std::collections::HashSet::new(),
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
            // Expansion computes a term, never validates the original syntax.
            // infer_type_kind_chirho still checks every original ascription;
            // no caller may use synonym expansion as its classifier proof.
            TypeChirho::KindAnnotChirho { type_chirho, .. } => {
                Self::collect_type_app_spine_chirho(type_chirho, args_chirho)
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
            TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho: inner_arg_chirho,
                span_chirho,
            } => TypeChirho::KindAppChirho {
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
            TypeChirho::KindAnnotChirho {
                type_chirho,
                kind_chirho,
                span_chirho,
            } => TypeChirho::KindAnnotChirho {
                type_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    type_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                kind_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    kind_chirho,
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
        let result_chirho = self.unify_family_kinds_chirho(
            &k1_applied_chirho,
            &k2_applied_chirho,
            context_chirho,
            span_chirho,
        );
        self.commit_kind_unification_chirho(result_chirho, context_chirho, span_chirho);
    }

    /// Only a checked substitution may enter the shared inference state. Kind
    /// ascription subsumption also uses this boundary after its local skolems
    /// have been checked for escape and instantiated for the actual use.
    fn commit_kind_unification_chirho(
        &mut self,
        result_chirho: Result<KindSubstChirho, KindErrorChirho>,
        context_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        match result_chirho {
            Ok(s_chirho) => {
                self.subst_chirho = s_chirho.compose_chirho(&self.subst_chirho);
                self.check_solved_kind_classifiers_chirho(&s_chirho, context_chirho, span_chirho);
            }
            Err(err_chirho) => {
                let (msg_chirho, span_chirho, code_chirho) = match err_chirho {
                    KindErrorChirho::ReductionLimitChirho { span_chirho } => (
                        "kind-family reduction exceeded its work limit".to_owned(),
                        span_chirho,
                        KIND_MISMATCH_CODE_CHIRHO,
                    ),
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
        AstKindChirho::ForallChirho { .. }
        | AstKindChirho::RequiredForallChirho { .. }
        | AstKindChirho::TypeSyntaxChirho(_)
        | AstKindChirho::KindAnnotChirho { .. } => {
            panic!("quantified or ascribed kind conversion requires the scoped kind context")
        }
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
        AstKindChirho::KindAppChirho(fun_chirho, arg_chirho) => KindChirho::KindAppChirho(
            Box::new(ast_kind_to_kind_chirho(fun_chirho)),
            Box::new(ast_kind_to_kind_chirho(arg_chirho)),
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
    /// Checked local heads, not naming exports; the driver applies visibility.
    pub contracts_chirho: HashMap<String, KindContractChirho>,
    pub associated_contracts_chirho: HashMap<String, KindContractChirho>,
    /// The kind environment after inference (type constructors → kinds).
    pub env_chirho: KindEnvChirho,
    /// Solved hidden arguments consumed by type inference, not discarded after checking.
    pub elaboration_chirho: KindElaborationChirho,
    /// Diagnostics (errors and warnings) from kind checking.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Run kind inference on a module's type declarations and type signatures.
pub fn infer_module_kinds_chirho(module_chirho: &ModuleChirho) -> KindResultChirho {
    infer_module_kinds_with_imports_chirho(module_chirho, &HashMap::new())
}

pub fn infer_module_kinds_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_chirho: &HashMap<String, KindContractChirho>,
) -> KindResultChirho {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    ctx_chirho.seed_imported_kind_contracts_chirho(imported_chirho);
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
    let class_defaults_chirho: HashMap<_, _> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|declaration_chirho| {
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                type_vars_chirho,
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                Some((
                    name_chirho.text_chirho(),
                    (
                        type_vars_chirho.as_slice(),
                        associated_tfs_chirho.as_slice(),
                    ),
                ))
            } else {
                None
            }
        })
        .collect();
    for (declaration_index_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
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
        ctx_chirho.check_associated_instance_equations_chirho(
            declaration_index_chirho,
            decl_chirho,
            class_defaults_chirho
                .get(class_chirho.text_chirho())
                .copied(),
        );
    }

    // Phase 2: Finalize — apply substitution and default unconstrained vars.
    ctx_chirho.finalize_chirho(poly_kinds_enabled_chirho);
    ctx_chirho.check_constraint_synonym_licenses_chirho(module_chirho);

    KindResultChirho {
        contracts_chirho: ctx_chirho.export_kind_contracts_chirho(module_chirho),
        associated_contracts_chirho: ctx_chirho
            .export_associated_kind_contracts_chirho(module_chirho),
        elaboration_chirho: ctx_chirho.finish_kind_elaboration_chirho(module_chirho),
        env_chirho: ctx_chirho.env_chirho,
        diagnostics_chirho: ctx_chirho.diagnostics_chirho,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[path = "kind_chirho/tests_chirho/mod_chirho.rs"]
mod tests_chirho;
