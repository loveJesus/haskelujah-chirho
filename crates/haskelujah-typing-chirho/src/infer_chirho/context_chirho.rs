// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The inference context: its construction from the seeded builtins and the imports, and the
//! finalization of its occurrence records into the result.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{
    InferResultChirho, MethodOccurrenceRecordChirho, evidence_chirho, seed_builtins_chirho,
};
use crate::class_chirho::ClassEnvChirho;
use crate::env_chirho::TyEnvChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;
use haskelujah_diagnostics_chirho::DiagnosticBundleChirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    pub fn new_chirho() -> Self {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();
        let fully_known_class_names_chirho: HashSet<String> =
            class_env_chirho.classes_chirho.keys().cloned().collect();
        // The seed producer's own output (row 686, C08).
        let seeded_class_names_chirho = fully_known_class_names_chirho.clone();
        seed_builtins_chirho(&mut env_chirho);
        let mut type_synonyms_chirho = HashMap::new();
        // Built-in type synonym: type String = [Char]
        type_synonyms_chirho.insert(
            "String".to_string(),
            (
                vec![],
                TyChirho::ListChirho(Box::new(TyChirho::char_chirho())),
            ),
        );
        // Built-in type synonym: type ShowS = String -> String
        type_synonyms_chirho.insert(
            "ShowS".to_string(),
            (
                vec![],
                TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho()),
            ),
        );
        // Built-in type synonym: type ReadS a = String -> [(a, String)]
        type_synonyms_chirho.insert(
            "ReadS".to_string(),
            (
                vec!["a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::ForallVarChirho("a".to_string()),
                        TyChirho::string_chirho(),
                    ]))),
                ),
            ),
        );
        // GHC.TypeLits: type Nat = Natural
        type_synonyms_chirho.insert(
            "Nat".to_string(),
            (vec![], TyChirho::ConChirho("Natural".to_string())),
        );
        // GHC.Generics metadata aliases:
        // type D1 = M1 D
        // type C1 = M1 C
        // type S1 = M1 S
        for (metadata_alias_chirho, metadata_tag_chirho) in [("D1", "D"), ("C1", "C"), ("S1", "S")]
        {
            let rhs_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("M1".to_owned())),
                Box::new(TyChirho::ConChirho(metadata_tag_chirho.to_owned())),
            );
            type_synonyms_chirho.insert(
                metadata_alias_chirho.to_string(),
                (vec![], rhs_chirho.clone()),
            );
            type_synonyms_chirho.insert(
                format!("GHC.Generics.{metadata_alias_chirho}"),
                (vec![], rhs_chirho),
            );
        }
        // type FilePath = String
        type_synonyms_chirho.insert("FilePath".to_string(), (vec![], TyChirho::string_chirho()));
        // Test.Hspec frontiers: model Spec/Expectation as IO () for type-checking
        // package test suites without implementing the Hspec runner.
        let hspec_action_ty_chirho = TyChirho::io_chirho(TyChirho::unit_chirho());
        type_synonyms_chirho.insert("Spec".to_string(), (vec![], hspec_action_ty_chirho.clone()));
        type_synonyms_chirho.insert("Expectation".to_string(), (vec![], hspec_action_ty_chirho));
        // Monad transformer aliases from transformers / mtl.
        type_synonyms_chirho.insert(
            "State".to_string(),
            (
                vec!["s".to_string(), "a".to_string()],
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("StateT".to_string())),
                            Box::new(TyChirho::ForallVarChirho("s".to_string())),
                        )),
                        Box::new(TyChirho::ConChirho("Identity".to_string())),
                    )),
                    Box::new(TyChirho::ForallVarChirho("a".to_string())),
                ),
            ),
        );
        // Template Haskell aliases
        type_synonyms_chirho.insert(
            "Kind".to_string(),
            (vec![], TyChirho::ConChirho("Type".to_string())),
        );
        type_synonyms_chirho.insert(
            "Pred".to_string(),
            (vec![], TyChirho::ConChirho("Type".to_string())),
        );
        type_synonyms_chirho.insert(
            "Cxt".to_string(),
            (
                vec![],
                TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Pred".to_string()))),
            ),
        );
        type_synonyms_chirho.insert(
            "TypeQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Type".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "ExpQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Exp".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "PatQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Pat".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "DecQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Dec".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "DecsQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ListChirho(Box::new(TyChirho::ConChirho(
                        "Dec".to_string(),
                    )))),
                ),
            ),
        );
        // Control.Monad.Signatures
        type_synonyms_chirho.insert(
            "CallCC".to_string(),
            (
                vec!["m".to_string(), "a".to_string(), "b".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::ForallVarChirho("a".to_string()),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ForallVarChirho("m".to_string())),
                                Box::new(TyChirho::ForallVarChirho("b".to_string())),
                            ),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ForallVarChirho("m".to_string())),
                            Box::new(TyChirho::ForallVarChirho("a".to_string())),
                        ),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Catch".to_string(),
            (
                vec!["e".to_string(), "m".to_string(), "a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::ForallVarChirho("e".to_string()),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ForallVarChirho("m".to_string())),
                                Box::new(TyChirho::ForallVarChirho("a".to_string())),
                            ),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ForallVarChirho("m".to_string())),
                            Box::new(TyChirho::ForallVarChirho("a".to_string())),
                        ),
                    ),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Listen".to_string(),
            (
                vec!["w".to_string(), "m".to_string(), "a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::ForallVarChirho("a".to_string()),
                            TyChirho::ForallVarChirho("w".to_string()),
                        ])),
                    ),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Pass".to_string(),
            (
                vec!["w".to_string(), "m".to_string(), "a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::ForallVarChirho("a".to_string()),
                            TyChirho::fun_chirho(
                                TyChirho::ForallVarChirho("w".to_string()),
                                TyChirho::ForallVarChirho("w".to_string()),
                            ),
                        ])),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                ),
            ),
        );
        // Built-in type synonym: type Buffer = Ptr Word8
        type_synonyms_chirho.insert(
            "Buffer".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Ptr".to_string())),
                    Box::new(TyChirho::ConChirho("Word8".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "CString".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Ptr".to_string())),
                    Box::new(TyChirho::ConChirho("CChar".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "CStringLen".to_string(),
            (
                vec![],
                TyChirho::TupleChirho(vec![
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Ptr".to_string())),
                        Box::new(TyChirho::ConChirho("CChar".to_string())),
                    ),
                    TyChirho::int_chirho(),
                ]),
            ),
        );
        // Built-in type synonym: type CI a = a
        type_synonyms_chirho.insert(
            "CI".to_string(),
            (
                vec!["a".to_string()],
                TyChirho::ForallVarChirho("a".to_string()),
            ),
        );
        // BufSize = Int (from Network.Socket.BufferPool)
        type_synonyms_chirho.insert("BufSize".to_string(), (vec![], TyChirho::int_chirho()));
        // Built-in http-types aliases
        type_synonyms_chirho.insert(
            "HeaderName".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("CI".to_string())),
                    Box::new(TyChirho::ConChirho("ByteString".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Header".to_string(),
            (
                vec![],
                TyChirho::TupleChirho(vec![
                    TyChirho::ConChirho("HeaderName".to_string()),
                    TyChirho::ConChirho("ByteString".to_string()),
                ]),
            ),
        );
        type_synonyms_chirho.insert(
            "ResponseHeaders".to_string(),
            (
                vec![],
                TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Header".to_string()))),
            ),
        );
        Self {
            next_var_chirho: 0,
            env_chirho,
            class_env_chirho,
            deferred_preds_chirho: Vec::new(),
            occurrence_captures_chirho: Vec::new(),
            occurrence_default_hints_chirho: HashMap::new(),
            occurrence_generalized_vars_chirho: HashSet::new(),
            inferred_dictionary_owners_chirho: HashMap::new(),
            occurrence_counters_chirho: HashMap::new(),
            literal_captures_chirho: Vec::new(),
            reference_captures_chirho: Vec::new(),
            recursive_references_chirho: Default::default(),
            skolem_pred_index_chirho: HashMap::new(),
            local_generalized_vars_chirho: HashSet::new(),
            local_instantiations_chirho: HashMap::new(),
            given_preds_chirho: Vec::new(),
            diagnostics_chirho: DiagnosticBundleChirho::empty_chirho(),
            type_synonyms_chirho,
            type_families_chirho: super::seed_type_family_registry_chirho(),
            family_module_name_chirho: "Main".to_owned(),
            family_scope_chirho: None,
            scoped_tyvars_chirho: HashMap::new(),
            checking_binding_pattern_annotations_chirho: false,
            next_skolem_chirho: 0,
            skolem_bindings_chirho: SubstChirho::empty_chirho(),
            instance_method_skolems_chirho: HashSet::new(),
            scoped_type_variables_chirho: true,
            relaxed_poly_rec_chirho: true,
            do_last_expected_chirho: None,
            given_rewrites_chirho: Vec::new(),
            deferred_equalities_chirho: Vec::new(),
            assoc_type_defaults_chirho: HashMap::new(),
            assoc_type_declared_params_chirho: HashMap::new(),
            tyvar_source_names_chirho: HashMap::new(),
            data_constructors_chirho: HashMap::new(),
            local_instances_chirho: Vec::new(),
            strict_positional_constructors_chirho: HashSet::new(),
            con_strict_fields_chirho: HashMap::new(),
            con_field_names_chirho: HashMap::new(),
            module_default_types_chirho: Vec::new(),
            overloaded_strings_chirho: false,
            fully_known_class_names_chirho,
            seeded_class_names_chirho,
            unmodeled_pattern_evidence_seen_chirho: false,
            context_constructors_chirho: HashSet::new(),
            constraint_generation_faithful_chirho: true,
            derived_instance_heads_chirho: HashSet::new(),
            safe_unqualified_imported_type_names_chirho: HashSet::new(),
            preferred_qualified_type_names_chirho: HashMap::new(),
            promotion_scope_chirho: None,
        }
    }

    pub fn new_with_imported_class_env_chirho(imported_class_env_chirho: &ClassEnvChirho) -> Self {
        let mut ctx_chirho = Self::new_chirho();
        if !imported_class_env_chirho.classes_chirho.is_empty()
            || !imported_class_env_chirho.instances_chirho.is_empty()
        {
            ctx_chirho.class_env_chirho = imported_class_env_chirho.clone();
            // The seeded env was replaced wholesale; the seeded classes'
            // instance inventories are no longer trustworthy, and imported
            // classes may have instances in modules we cannot see. Only
            // classes declared in THIS module stay fully known.
            ctx_chirho.fully_known_class_names_chirho.clear();
            ctx_chirho.seeded_class_names_chirho.clear();
        }
        ctx_chirho
    }

    /// Evidence-threading P2: finalize raw occurrence captures against the
    /// composed substitution (unification + Report defaulting), keeping only
    /// records whose predicate type resolved to a concrete instance-head key.
    pub(super) fn finalize_occurrence_records_chirho(
        &self,
        final_subst_chirho: &SubstChirho,
    ) -> Vec<MethodOccurrenceRecordChirho> {
        fn head_key_chirho(ty_chirho: &TyChirho) -> Option<String> {
            match ty_chirho {
                TyChirho::ConChirho(name_chirho) => Some(name_chirho.clone()),
                TyChirho::ListChirho(_) => Some("[]".to_string()),
                TyChirho::TupleChirho(items_chirho) => Some(format!(
                    "({})",
                    ",".repeat(items_chirho.len().saturating_sub(1))
                )),
                TyChirho::AppChirho(fun_chirho, _) => head_key_chirho(fun_chirho),
                _ => None,
            }
        }
        fn default_occurrence_key_chirho(class_name_chirho: &str) -> Option<String> {
            match class_name_chirho {
                "Fractional" | "Floating" | "RealFrac" | "RealFloat" => Some("Double".to_string()),
                "Num" | "Integral" | "Real" | "Enum" | "Bounded" | "Eq" | "Ord" | "Show"
                | "Read" => Some("Int".to_string()),
                _ => None,
            }
        }
        self.occurrence_captures_chirho
            .iter()
            .filter_map(
                |evidence_chirho::OccurrenceCaptureChirho {
                     name_chirho,
                     ordinal_chirho,
                     class_name_chirho: class_chirho,
                     ty_chirho,
                     span_chirho,
                     origin_chirho,
                 }| {
                    let resolved_chirho = final_subst_chirho.apply_ty_chirho(ty_chirho);
                    let hinted_key_chirho = match &resolved_chirho {
                        TyChirho::VarChirho(var_chirho) => self
                            .occurrence_default_hints_chirho
                            .get(var_chirho)
                            .cloned(),
                        _ => None,
                    };
                    if hinted_key_chirho.is_none()
                        && matches!(
                            &resolved_chirho,
                            TyChirho::VarChirho(var_chirho)
                                if self.occurrence_generalized_vars_chirho.contains(var_chirho)
                        )
                    {
                        // A local binding keeps its single-instantiation proof. A
                        // top-level binding takes its final scheme's agreed index
                        // (workflow:
                        // language-features-chirho/dictionary-evidence-chirho).
                        if let TyChirho::VarChirho(var_chirho) = &resolved_chirho
                            && let Some(key_chirho) = self
                                .local_specialization_key_chirho(*var_chirho, final_subst_chirho)
                                .or_else(|| {
                                    (!self.local_generalized_vars_chirho.contains(var_chirho))
                                        .then(|| {
                                            self.inferred_dictionary_owner_key_chirho(
                                                class_chirho,
                                                *var_chirho,
                                            )
                                        })
                                        .flatten()
                                })
                        {
                            return Some(MethodOccurrenceRecordChirho {
                                name_chirho: name_chirho.clone(),
                                ordinal_chirho: *ordinal_chirho,
                                class_name_chirho: class_chirho.clone(),
                                ty_key_chirho: key_chirho,
                                ground_pred_chirho: None,
                                span_chirho: *span_chirho,
                                origin_chirho: *origin_chirho,
                            });
                        }
                        return None;
                    }
                    // A method used at a rigid variable of the enclosing signature
                    // (`x + 1` inside `f :: Num a => a -> a`) has no instance to
                    // name: its evidence is the binding's own dictionary parameter.
                    // Defaulting it to `Int` here dispatched `f 2.5` to the Int row.
                    // An instance method's skolem is no such parameter and keys
                    // as the open variable it replaced, below.
                    // workflow: language-features-chirho/dictionary-evidence-chirho
                    if let TyChirho::ForallVarChirho(skolem_chirho) = &resolved_chirho
                        && let Some(key_chirho) =
                            self.own_parameter_key_chirho(class_chirho, skolem_chirho)
                    {
                        return Some(MethodOccurrenceRecordChirho {
                            name_chirho: name_chirho.clone(),
                            ordinal_chirho: *ordinal_chirho,
                            class_name_chirho: class_chirho.clone(),
                            ty_key_chirho: key_chirho,
                            ground_pred_chirho: None,
                            span_chirho: *span_chirho,
                            origin_chirho: *origin_chirho,
                        });
                    }
                    // A concrete type keys whole for Show, Ord and Eq, as their
                    // structured rows are named (`$prim_Eq_==_Maybe Int`, row 603).
                    let resolved_structured_key_chirho =
                        (matches!(class_chirho.as_str(), "Show" | "Ord" | "Eq")
                            && !resolved_chirho.contains_var_chirho())
                        .then(|| {
                            evidence_chirho::concrete_evidence_key_chirho(&resolved_chirho, false)
                        })
                        .flatten();
                    let key_chirho = resolved_structured_key_chirho
                        .or_else(|| evidence_chirho::ground_list_key_chirho(&resolved_chirho))
                        .or_else(|| head_key_chirho(&resolved_chirho))
                        .or(hinted_key_chirho)
                        .or_else(|| default_occurrence_key_chirho(class_chirho))?;
                    Some(MethodOccurrenceRecordChirho {
                        name_chirho: name_chirho.clone(),
                        ordinal_chirho: *ordinal_chirho,
                        class_name_chirho: class_chirho.clone(),
                        ty_key_chirho: key_chirho,
                        ground_pred_chirho: evidence_chirho::ground_pred_chirho(
                            self,
                            class_chirho,
                            &resolved_chirho,
                            &[],
                        ),
                        span_chirho: *span_chirho,
                        origin_chirho: *origin_chirho,
                    })
                },
            )
            .collect()
    }

    /// Consume the context and return the final result.
    pub fn finish_chirho(self) -> InferResultChirho {
        InferResultChirho {
            subst_chirho: SubstChirho::empty_chirho(),
            env_chirho: self.env_chirho,
            class_env_chirho: self.class_env_chirho,
            type_families_chirho: self.type_families_chirho,
            diagnostics_chirho: self.diagnostics_chirho,
            method_occurrences_chirho: Vec::new(),
            method_occurrence_totals_chirho: HashMap::new(),
            literal_evidence_chirho: HashMap::new(),
            literal_evidence_by_origin_chirho: HashMap::new(),
            reference_evidence_chirho: HashMap::new(),
            reference_evidence_by_origin_chirho: HashMap::new(),
            promotion_scope_chirho: self.promotion_scope_chirho,
            family_scope_chirho: self.family_scope_chirho,
        }
    }
}
