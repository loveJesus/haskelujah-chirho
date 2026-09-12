// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Module inference inputs and result finalization. Workflow: declaration-kinds-chirho.
use super::*;

/// Imported semantic contracts and the already-checked local kind elaboration.
/// The driver hands its actual phase result over; direct type-inference callers
/// may request elaboration locally by leaving `kind_elaboration_chirho` absent.
pub struct InferInputsChirho<'input_chirho> {
    pub imported_types_chirho: &'input_chirho HashMap<String, SchemeChirho>,
    pub imported_type_synonyms_chirho: &'input_chirho HashMap<String, (Vec<String>, TypeChirho)>,
    pub imported_closed_synonyms_chirho: &'input_chirho HashMap<String, TypeSynonymChirho>,
    pub imported_type_families_chirho: &'input_chirho TypeFamilyEnvChirho,
    pub imported_class_env_chirho: &'input_chirho ClassEnvChirho,
    pub imported_record_field_names_chirho: &'input_chirho HashMap<String, Vec<String>>,
    pub safe_unqualified_imported_type_names_chirho: &'input_chirho HashSet<String>,
    pub preferred_qualified_type_names_chirho: &'input_chirho HashMap<String, String>,
    pub kind_elaboration_chirho: Option<crate::kind_chirho::KindElaborationChirho>,
}

/// Run type inference on a module. This is the main entry point.
pub fn infer_module_chirho(module_chirho: &ModuleChirho) -> InferResultChirho {
    let safe_unqualified_imported_type_names_chirho = HashSet::new();
    let preferred_qualified_type_names_chirho = HashMap::new();
    infer_module_with_imports_and_type_synonyms_chirho(
        module_chirho,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &safe_unqualified_imported_type_names_chirho,
        &preferred_qualified_type_names_chirho,
    )
}

/// Run type inference on a module with pre-seeded type schemes from imported
/// modules. Each entry maps a name (e.g. `"add1"`) to the type scheme that
/// was inferred in the exporting module.
pub fn infer_module_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
) -> InferResultChirho {
    let safe_unqualified_imported_type_names_chirho = HashSet::new();
    let preferred_qualified_type_names_chirho = HashMap::new();
    infer_module_with_imports_and_type_synonyms_chirho(
        module_chirho,
        imported_types_chirho,
        &HashMap::new(),
        &HashMap::new(),
        &safe_unqualified_imported_type_names_chirho,
        &preferred_qualified_type_names_chirho,
    )
}

pub(super) fn is_placeholder_import_scheme_chirho(scheme_chirho: &SchemeChirho) -> bool {
    if !scheme_chirho.preds_chirho.is_empty() || scheme_chirho.vars_chirho.len() != 1 {
        return false;
    }
    match &scheme_chirho.ty_chirho {
        TyChirho::VarChirho(var_chirho) => {
            *var_chirho == scheme_chirho.vars_chirho[0] && var_chirho.0 >= 9000
        }
        _ => false,
    }
}

/// Run type inference on a module with pre-seeded imported value schemes and
/// imported type synonym definitions from upstream modules.
pub fn infer_module_with_imports_and_type_synonyms_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
    imported_type_synonyms_chirho: &HashMap<String, (Vec<String>, TypeChirho)>,
    imported_record_field_names_chirho: &HashMap<String, Vec<String>>,
    safe_unqualified_imported_type_names_chirho: &HashSet<String>,
    preferred_qualified_type_names_chirho: &HashMap<String, String>,
) -> InferResultChirho {
    let imported_type_families_chirho = TypeFamilyEnvChirho::new();
    infer_module_with_imports_type_synonyms_and_families_chirho(
        module_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        &imported_type_families_chirho,
        imported_record_field_names_chirho,
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    )
}

pub fn infer_module_with_imports_type_synonyms_and_families_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
    imported_type_synonyms_chirho: &HashMap<String, (Vec<String>, TypeChirho)>,
    imported_type_families_chirho: &TypeFamilyEnvChirho,
    imported_record_field_names_chirho: &HashMap<String, Vec<String>>,
    safe_unqualified_imported_type_names_chirho: &HashSet<String>,
    preferred_qualified_type_names_chirho: &HashMap<String, String>,
) -> InferResultChirho {
    let imported_class_env_chirho = ClassEnvChirho::new_chirho();
    infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
        module_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
        &imported_class_env_chirho,
        imported_record_field_names_chirho,
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    )
}

pub fn infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
    imported_type_synonyms_chirho: &HashMap<String, (Vec<String>, TypeChirho)>,
    imported_type_families_chirho: &TypeFamilyEnvChirho,
    imported_class_env_chirho: &ClassEnvChirho,
    imported_record_field_names_chirho: &HashMap<String, Vec<String>>,
    safe_unqualified_imported_type_names_chirho: &HashSet<String>,
    preferred_qualified_type_names_chirho: &HashMap<String, String>,
) -> InferResultChirho {
    infer_module_with_inputs_chirho(
        module_chirho,
        InferInputsChirho {
            imported_types_chirho,
            imported_type_synonyms_chirho,
            imported_closed_synonyms_chirho: &HashMap::new(),
            imported_type_families_chirho,
            imported_class_env_chirho,
            imported_record_field_names_chirho,
            safe_unqualified_imported_type_names_chirho,
            preferred_qualified_type_names_chirho,
            kind_elaboration_chirho: None,
        },
    )
}

pub fn infer_module_with_inputs_chirho(
    module_chirho: &ModuleChirho,
    inputs_chirho: InferInputsChirho<'_>,
) -> InferResultChirho {
    let InferInputsChirho {
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_closed_synonyms_chirho,
        imported_type_families_chirho,
        imported_class_env_chirho,
        imported_record_field_names_chirho,
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
        kind_elaboration_chirho,
    } = inputs_chirho;
    let mut ctx_chirho =
        InferCtxChirho::new_with_imported_class_env_chirho(imported_class_env_chirho);
    ctx_chirho.kind_elaboration_chirho = kind_elaboration_chirho;
    ctx_chirho.set_imported_type_name_preferences_chirho(
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    );
    ctx_chirho.overloaded_strings_chirho = module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| extension_chirho == "OverloadedStrings");
    // DETERMINISM: these seeding loops iterate `HashMap`s and MUTATE inference
    // state, so a per-process hash seed would otherwise make the resulting
    // environment — and therefore the accept/reject decision — differ between
    // runs of the same binary on the same source. Sort by key so the order is a
    // function of the program, not of the hasher.
    // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
    let mut sorted_type_synonyms_chirho: Vec<_> = imported_type_synonyms_chirho.iter().collect();
    sorted_type_synonyms_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (name_chirho, (params_chirho, rhs_ast_chirho)) in sorted_type_synonyms_chirho {
        if imported_closed_synonyms_chirho.contains_key(name_chirho) {
            continue;
        }
        let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(rhs_ast_chirho, params_chirho);
        ctx_chirho.register_type_synonym_chirho(
            name_chirho.clone(),
            params_chirho.clone(),
            rhs_ty_chirho,
        );
    }
    let mut sorted_closed_synonyms_chirho: Vec<_> =
        imported_closed_synonyms_chirho.iter().collect();
    sorted_closed_synonyms_chirho
        .sort_by(|left_chirho, right_chirho| left_chirho.0.cmp(right_chirho.0));
    for (name_chirho, synonym_chirho) in sorted_closed_synonyms_chirho {
        let synonym_chirho = synonym_chirho.map_constructor_names_chirho(&mut |name_chirho| {
            ctx_chirho.normalize_imported_type_name_chirho(name_chirho)
        });
        let normalized_name_chirho = ctx_chirho.normalize_imported_type_name_chirho(name_chirho);
        ctx_chirho
            .type_synonyms_chirho
            .insert(normalized_name_chirho, synonym_chirho.clone());
        ctx_chirho
            .type_synonyms_chirho
            .insert(name_chirho.clone(), synonym_chirho);
    }
    let mut sorted_type_families_chirho: Vec<_> = imported_type_families_chirho.iter().collect();
    sorted_type_families_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (family_name_chirho, equations_chirho) in sorted_type_families_chirho {
        ctx_chirho.register_elaborated_type_family_chirho(
            family_name_chirho.clone(),
            equations_chirho.clone(),
        );
    }
    // Seed the type environment with imported type schemes, but only if
    // placeholder imports do not override precise built-ins, while real
    // imported schemes from previously checked modules do override built-ins.
    // This lets modules like Text.Parsec.Combinator see Text.Parsec.Prim.try
    // instead of the unrelated builtin Control.Exception.try.
    // DETERMINISM: order matters here more than anywhere else in this function —
    // whether a scheme is bound depends on `lookup_chirho(..).is_none()`, i.e. on
    // what was already bound. Iterating a hash map meant the winner between a
    // placeholder and a real import could change from run to run.
    let mut sorted_imported_types_chirho: Vec<_> = imported_types_chirho.iter().collect();
    sorted_imported_types_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (name_chirho, scheme_chirho) in sorted_imported_types_chirho {
        let should_override_chirho = !is_placeholder_import_scheme_chirho(scheme_chirho);
        if should_override_chirho || ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_none() {
            ctx_chirho
                .env_chirho
                .bind_chirho(name_chirho.clone(), scheme_chirho.clone());
        }
    }
    let mut sorted_record_fields_chirho: Vec<_> =
        imported_record_field_names_chirho.iter().collect();
    sorted_record_fields_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (constructor_name_chirho, field_names_chirho) in sorted_record_fields_chirho {
        ctx_chirho
            .con_field_names_chirho
            .insert(constructor_name_chirho.clone(), field_names_chirho.clone());
    }
    let subst_chirho = ctx_chirho.infer_module_chirho(module_chirho);
    let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
    if !default_subst_chirho.is_empty_chirho() {
        ctx_chirho.apply_subst_all_chirho(&default_subst_chirho);
    }
    let composed_subst_chirho = default_subst_chirho.compose_chirho(&subst_chirho);
    // Evidence-threading P2: finalize occurrence records through the composed
    // substitution (so Report defaulting from check_deferred_preds_chirho is
    // reflected) before the context is consumed.
    let method_occurrences_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&composed_subst_chirho);
    let method_occurrence_totals_chirho = ctx_chirho.occurrence_counters_chirho.clone();
    let literal_evidence_chirho =
        ctx_chirho.finalize_literal_evidence_chirho(&composed_subst_chirho);
    let reference_evidence_chirho =
        ctx_chirho.finalize_reference_evidence_chirho(&composed_subst_chirho);
    let mut result_chirho = ctx_chirho.finish_chirho();
    result_chirho.subst_chirho = composed_subst_chirho;
    result_chirho.method_occurrences_chirho = method_occurrences_chirho;
    result_chirho.method_occurrence_totals_chirho = method_occurrence_totals_chirho;
    result_chirho.literal_evidence_chirho = literal_evidence_chirho;
    result_chirho.reference_evidence_chirho = reference_evidence_chirho;
    result_chirho
}
