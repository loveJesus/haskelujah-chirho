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
        .classes_chirho
        .values()
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
        },
    );
    let reference_evidence_chirho = evidence_join_chirho::join_reference_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.reference_occurrence_spans_chirho,
        &desugar_output_chirho.occurrence_provenance_chirho,
    );
    let mut occurrence_evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.method_occurrences_chirho,
        &desugar_output_chirho.literal_occurrence_spans_chirho,
        &desugar_output_chirho.method_occurrence_spans_chirho,
        &desugar_output_chirho.occurrence_provenance_chirho,
    );
    // Range syntax selected Enum itself, independently of a lexical namesake.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    occurrence_evidence_chirho.retain(|id_chirho, _| {
        !desugar_output_chirho
            .range_occurrence_spans_chirho
            .contains_key(id_chirho)
    });
    occurrence_evidence_chirho.extend(evidence_join_chirho::join_range_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.method_occurrences_chirho,
        &desugar_output_chirho.range_occurrence_spans_chirho,
    ));

    // Phase 5.5: Dictionary-passing transform (desugar typeclass constraints)
    let con_types_chirho = build_con_type_map_chirho(&module_chirho);
    // The builtin classes the module's class spellings name (K01): the frontend built the
    // same scope from the same declarations, imports and interfaces before deriving, and
    // deriving adds only instances. The newtype inventory and the derived Show and Enum
    // metadata judge each derived class by it, exactly as the deriving pass routed it (row 694).
    // workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    let builtin_class_scope_chirho =
        haskelujah_naming_chirho::build_builtin_class_scope_chirho(&module_chirho, ifaces_chirho);
    let newtype_info_chirho = build_newtype_info_chirho(
        &module_chirho,
        &builtin_class_scope_chirho,
        &infer_result_chirho.gnd_coercion_proofs_chirho,
        &desugar_output_chirho.default_method_rows_chirho,
    );
    let mut newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
        .values()
        .map(|info_chirho| info_chirho.constructor_chirho.clone())
        .collect();
    // The constructors of newtypes that take a class through the newtype strategy. Core
    // erases them, so the representation's rows receive the field itself on every backend
    // (row 694). workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    let gnd_newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
        .values()
        .filter(|info_chirho| !info_chirho.newtype_derived_classes_chirho.is_empty())
        .map(|info_chirho| info_chirho.constructor_chirho.clone())
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
        haskelujah_typing_chirho::deriving_chirho::derived_show_metadata_with_builtins_chirho(
            &module_chirho,
            &builtin_class_scope_chirho,
        );
    let derived_enums_chirho =
        haskelujah_typing_chirho::deriving_chirho::derived_enum_metadata_with_builtins_chirho(
            &module_chirho,
            &builtin_class_scope_chirho,
        );
    let dict_result_chirho = haskelujah_core_chirho::dict_pass_module_full_with_evidence_chirho(
        &desugar_output_chirho.module_chirho,
        desugar_output_chirho.names_chirho,
        &infer_result_chirho.env_chirho,
        &infer_result_chirho.class_env_chirho,
        con_types_chirho,
        newtype_info_chirho,
        extra_dict_param_names_chirho,
        desugar_output_chirho.method_occurrences_chirho,
        occurrence_evidence_chirho,
        reference_evidence_chirho,
        derived_shows_chirho,
        derived_enums_chirho,
        desugar_output_chirho.operator_occurrences_chirho,
    );
    // A GND instance whose representation's methods the dictionary pass could not build is
    // refused, never aliased to a missing method (row 694).
    // workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    if !dict_result_chirho.unsupported_chirho.is_empty() {
        return Err(unsupported_gnd_diagnostics_chirho(
            &dict_result_chirho.unsupported_chirho,
        ));
    }
    let mut core_chirho = dict_result_chirho.module_chirho;
    haskelujah_core_chirho::simplify_chirho::erase_newtype_constructors_chirho(
        &mut core_chirho,
        &gnd_newtype_cons_chirho,
    );

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
        gnd_newtype_cons_chirho,
    })
}

/// `E0209`, the code declaration validity reports a refused declaration under.
const UNSUPPORTED_GND_CODE_CHIRHO: u16 = 209;

/// The dictionary pass's unsupported GND instances, as errors (row 694).
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
fn unsupported_gnd_diagnostics_chirho(reasons_chirho: &[String]) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    for reason_chirho in reasons_chirho {
        diagnostics_chirho.push_chirho(
            haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                    UNSUPPORTED_GND_CODE_CHIRHO,
                ),
                reason_chirho.clone(),
                haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
            )
            .with_note_chirho("Haskelujah does not support this yet"),
        );
    }
    diagnostics_chirho
}
