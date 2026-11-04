// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The backend pipeline, moved from the driver root (row 691, L7): phases 5 through 7 for a module
//! the frontend has already compiled (desugar, dictionary passing, simplification, LLVM IR and Wasm
//! bytes), producing its compile result.

use super::*;

/// Run the back-end pipeline phases (5 through 7) on an already-front-end-compiled
/// module, producing a [`CompileResultChirho`].
///
/// Phases: desugar → dict pass → simplify → LLVM IR → Wasm bytes.
pub(crate) fn compile_backend_chirho(
    module_chirho: ModuleChirho,
    infer_result_chirho: InferResultChirho,
    extra_dict_param_names_chirho: std::collections::HashSet<String>,
    ifaces_chirho: &[ModuleIfaceChirho],
    program_modules_chirho: &std::collections::HashSet<String>,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    if infer_result_chirho
        .non_deferrable_diagnostics_chirho
        .has_errors_chirho()
    {
        return Err(infer_result_chirho.non_deferrable_diagnostics_chirho);
    }
    // Phase 5: Desugar AST → Core IR (evidence-threading P2b: with
    // per-occurrence ids for the method-occurrence names).
    // Dictionary evidence: every name whose scheme carries class predicates
    // gets an occurrence id per reference, so the checker's per-reference
    // evidence can reach the dictionary pass.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    // Class selectors are not references to constrained functions and are not
    // minted here: the method-occurrence names (the standard set and this
    // module's own single-parameter class methods) get their ids from the
    // method-occurrence mechanism, and every other selector keeps its canonical id.
    // A source-owned value with that spelling remains a constrained reference (663/664).
    // workflow: language-features-chirho/own-class-method-occurrences-chirho
    let class_method_names_chirho: std::collections::HashSet<&String> = infer_result_chirho
        .class_env_chirho
        .classes_chirho()
        .values_chirho()
        .flat_map(|class_chirho| class_chirho.methods_chirho.keys())
        .collect();
    let import_suppliers_chirho = native_provenance_chirho::import_suppliers_chirho(
        &module_chirho,
        ifaces_chirho,
        program_modules_chirho,
    );
    let mut source_value_names_chirho =
        evidence_join_chirho::source_value_names_chirho(&module_chirho);
    source_value_names_chirho.extend(
        import_suppliers_chirho
            .value_names_chirho()
            .map(str::to_string),
    );
    let constrained_names_chirho: std::collections::HashSet<String> = infer_result_chirho
        .env_chirho
        .all_bindings_chirho()
        .into_iter()
        .filter(|(name_chirho, scheme_chirho)| {
            !scheme_chirho.preds_chirho.is_empty()
                && (!class_method_names_chirho.contains(name_chirho)
                    || source_value_names_chirho.contains(*name_chirho))
        })
        .map(|(name_chirho, _scheme_chirho)| name_chirho.clone())
        .collect();
    // Constructor arities, imported constructors included: `C {}` on a
    // positional constructor fills every argument with a missing-field thunk.
    // workflow: language-features-chirho/rigid-type-variables-chirho (records)
    let constructor_arities_chirho: std::collections::HashMap<String, usize> = infer_result_chirho
        .env_chirho
        .all_bindings_chirho()
        .into_iter()
        .filter(|(name_chirho, _scheme_chirho)| {
            name_chirho
                .chars()
                .next()
                .is_some_and(|first_chirho| first_chirho.is_uppercase())
        })
        .map(|(name_chirho, scheme_chirho)| {
            (name_chirho.clone(), scheme_chirho.spine_arity_chirho())
        })
        .collect();
    let desugar_output_chirho = haskelujah_core_chirho::desugar_module_with_inputs_chirho(
        &module_chirho,
        haskelujah_core_chirho::DesugarInputsChirho {
            method_names_chirho: evidence_join_chirho::method_occurrence_names_chirho(
                &module_chirho,
            ),
            constrained_names_chirho,
            constructor_arities_chirho,
            // A reference to a value another module of this program defines takes its
            // definer's identity, seen through any re-export (rows 629 and 642).
            // workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
            import_suppliers_chirho,
            promotion_scope_chirho: infer_result_chirho.promotion_scope_chirho.clone(),
            type_constructor_scope_chirho: infer_result_chirho
                .type_constructor_scope_chirho
                .clone(),
        },
    )
    .map_err(|error_chirho| {
        let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
        diagnostics_chirho.push_chirho(
            haskelujah_diagnostics_chirho::DiagnosticChirho::error_chirho(
                error_chirho.to_string(),
                error_chirho.span_chirho(),
            ),
        );
        diagnostics_chirho
    })?;
    let reference_evidence_chirho = evidence_join_chirho::join_reference_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.reference_occurrence_spans_chirho,
        &desugar_output_chirho.occurrence_provenance_chirho,
    );
    // One identity decision per occurrence gives both its key and its ground goals (S1).
    // workflow: language-features-chirho/dictionary-evidence-chirho
    let occurrence_proofs_chirho = evidence_join_chirho::join_occurrence_proofs_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.method_occurrences_chirho,
        &desugar_output_chirho.literal_occurrence_spans_chirho,
        &desugar_output_chirho.method_occurrence_spans_chirho,
        &desugar_output_chirho.occurrence_provenance_chirho,
    );
    let mut occurrence_evidence_chirho =
        evidence_join_chirho::occurrence_keys_chirho(&occurrence_proofs_chirho);
    let mut ground_goals_chirho =
        evidence_join_chirho::occurrence_ground_goals_chirho(&occurrence_proofs_chirho);
    // Range syntax selected Enum itself, independently of a lexical namesake.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    occurrence_evidence_chirho.retain(|id_chirho, _| {
        !desugar_output_chirho
            .range_occurrence_spans_chirho
            .contains_key(id_chirho)
    });
    // A range keeps its own Enum route; the conditional-dictionary route claims none.
    ground_goals_chirho.retain(|id_chirho, _| {
        !desugar_output_chirho
            .range_occurrence_spans_chirho
            .contains_key(id_chirho)
    });
    for (occ_id_chirho, reference_goals_chirho) in
        evidence_join_chirho::join_reference_ground_goals_chirho(
            &infer_result_chirho,
            &desugar_output_chirho.reference_occurrence_spans_chirho,
            &desugar_output_chirho.occurrence_provenance_chirho,
        )
    {
        let goals_chirho = ground_goals_chirho.entry(occ_id_chirho).or_default();
        for goal_chirho in reference_goals_chirho {
            if !goals_chirho.contains(&goal_chirho) {
                goals_chirho.push(goal_chirho);
            }
        }
    }
    occurrence_evidence_chirho.extend(evidence_join_chirho::join_range_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.method_occurrences_chirho,
        &desugar_output_chirho.range_occurrence_spans_chirho,
    ));

    // Phase 5.5: Dictionary-passing transform (desugar typeclass constraints)
    let con_types_chirho = build_con_type_map_chirho(&module_chirho);
    let newtype_info_chirho = build_newtype_info_chirho(
        &module_chirho,
        infer_result_chirho.promotion_scope_chirho.as_deref(),
    );
    let mut newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
        .values()
        .map(|(con_name_chirho, _)| con_name_chirho.clone())
        .collect();
    if !con_types_chirho.contains_key("Identity")
        && module_chirho.imports_chirho.iter().any(|import_chirho| {
            matches!(
                import_chirho.module_chirho.full_name_chirho().as_str(),
                "Data.Functor.Identity" | "Control.Monad.Identity"
            )
        })
    {
        newtype_cons_chirho.insert("Identity".to_string());
    }
    if !con_types_chirho.contains_key("Const")
        && module_chirho.imports_chirho.iter().any(|import_chirho| {
            import_chirho.module_chirho.full_name_chirho() == "Data.Functor.Const"
        })
    {
        newtype_cons_chirho.insert("Const".to_string());
    }
    if !con_types_chirho.contains_key("Down")
        && module_chirho
            .imports_chirho
            .iter()
            .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Data.Ord")
    {
        newtype_cons_chirho.insert("Down".to_string());
    }
    if module_chirho
        .imports_chirho
        .iter()
        .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Data.Semigroup")
    {
        for con_name_chirho in ["Min", "Max", "Endo"] {
            if !con_types_chirho.contains_key(con_name_chirho) {
                newtype_cons_chirho.insert(con_name_chirho.to_string());
            }
        }
    }
    if module_chirho
        .imports_chirho
        .iter()
        .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Data.Monoid")
    {
        for con_name_chirho in ["First", "Last", "Endo"] {
            if !con_types_chirho.contains_key(con_name_chirho) {
                newtype_cons_chirho.insert(con_name_chirho.to_string());
            }
        }
    }
    // Constructor metadata for every type that derives Show (row 533) or Enum (row 669).
    // workflow: print-show-evidence-chirho
    let derived_shows_chirho =
        haskelujah_typing_chirho::deriving_chirho::derived_show_metadata_chirho(&module_chirho);
    let derived_enums_chirho =
        haskelujah_typing_chirho::deriving_chirho::derived_enum_metadata_chirho(&module_chirho);
    // The checker's typed instance per source declaration, every span kept (S3).
    // workflow: language-features-chirho/dictionary-evidence-chirho
    let instance_declarations_chirho: Vec<haskelujah_core_chirho::InstanceDeclarationChirho> =
        infer_result_chirho
            .source_instances_chirho
            .iter()
            .map(|(span_chirho, instance_chirho)| {
                haskelujah_core_chirho::InstanceDeclarationChirho {
                    span_chirho: *span_chirho,
                    instance_chirho: instance_chirho.clone(),
                }
            })
            .collect();
    let dict_result_chirho = haskelujah_core_chirho::dict_pass_module_with_inputs_chirho(
        &desugar_output_chirho.module_chirho,
        haskelujah_core_chirho::DictPassInputsChirho {
            names_chirho: desugar_output_chirho.names_chirho,
            type_env_chirho: &infer_result_chirho.env_chirho,
            class_env_chirho: &infer_result_chirho.class_env_chirho,
            con_types_chirho,
            newtype_info_chirho,
            extra_dict_param_names_chirho,
            evidence_chirho: haskelujah_core_chirho::DictPassEvidenceChirho {
                method_occurrence_canon_chirho: desugar_output_chirho.method_occurrences_chirho,
                occurrence_evidence_chirho,
                reference_evidence_chirho,
                derived_shows_chirho,
                derived_enums_chirho,
                operator_occurrences_chirho: desugar_output_chirho.operator_occurrences_chirho,
                ground_goals_chirho,
                instance_declarations_chirho,
                instance_row_keys_chirho: desugar_output_chirho.instance_row_keys_chirho,
            },
        },
    );
    // A ground goal no route served and the pass refused by name is an error at its
    // occurrence, with promoted and family keys written as people read them (S2).
    // workflow: language-features-chirho/dictionary-evidence-chirho
    if !dict_result_chirho.diagnostics_chirho.is_empty() {
        let mut bundle_chirho = DiagnosticBundleChirho::new_chirho(
            dict_result_chirho
                .diagnostics_chirho
                .iter()
                .map(|diagnostic_chirho| {
                    let message_chirho = diagnostic_chirho.to_string();
                    match desugar_output_chirho
                        .method_occurrence_spans_chirho
                        .get(&diagnostic_chirho.occurrence_chirho)
                        .or_else(|| {
                            desugar_output_chirho
                                .reference_occurrence_spans_chirho
                                .get(&diagnostic_chirho.occurrence_chirho)
                        }) {
                        Some(span_chirho) => {
                            DiagnosticChirho::error_chirho(message_chirho, *span_chirho)
                        }
                        None => DiagnosticChirho::error_no_span_chirho(message_chirho),
                    }
                })
                .collect(),
        );
        bundle_chirho.map_text_chirho(
            haskelujah_ast_chirho::promotion_chirho::readable_promoted_keys_chirho,
        );
        let families_chirho = &infer_result_chirho.type_families_chirho;
        bundle_chirho
            .map_text_chirho(|text_chirho| families_chirho.readable_keys_chirho(text_chirho));
        return Err(bundle_chirho);
    }
    let core_chirho = dict_result_chirho.module_chirho;

    // Phase 6: Core-to-Core simplification (beta reduction, dead code, case-of-known)
    let config_chirho = SimplifyConfigChirho::default();
    let mut core_chirho = simplify_module_chirho(&core_chirho, &config_chirho);

    // Phase 6.5: Native provenance — the module's entry candidate and its builtin
    // references, from its imports (row 628).
    // workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    native_provenance_chirho::record_module_provenance_chirho(
        &module_chirho,
        &mut core_chirho,
        ifaces_chirho,
        program_modules_chirho,
    );

    // Phase 7: Backend lowering
    let llvm_ir_chirho = compile_core_to_llvm_chirho(&core_chirho);
    let wasm_bytes_chirho = compile_core_to_wasm_chirho(&core_chirho);

    Ok(CompileResultChirho {
        module_chirho,
        core_chirho,
        llvm_ir_chirho,
        wasm_bytes_chirho,
        newtype_cons_chirho,
    })
}
