// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual frontend admission/restore controls with a local resource policy.
//! Resource injection is an internal safety test, not a GHC-output comparison.
use crate::{
    ImportedTypeSynonymsChirho, run_frontend_with_type_synonyms_families_and_class_env_chirho,
    seed_builtin_type_families_chirho,
};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use std::collections::HashMap;

const REAL_DEFER_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# OPTIONS_GHC -fdefer-type-errors #-}
module Main where
main :: IO ()
main = print (42 :: Int)
"#;
const COMMENT_DEFER_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- The existing driver sees -fdefer-type-errors in this comment.
module Main where
main :: IO ()
main = print (42 :: Int)
"#;
const MANY_METHODS_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ManyMethodsChirho where
class ManyChirho a where
  method0Chirho :: a -> Int
  method1Chirho :: a -> Int
  method2Chirho :: a -> Int
  method3Chirho :: a -> Int
  method4Chirho :: a -> Int
  method5Chirho :: a -> Int
  method6Chirho :: a -> Int
  method7Chirho :: a -> Int
  method8Chirho :: a -> Int
  method9Chirho :: a -> Int
  method10Chirho :: a -> Int
  method11Chirho :: a -> Int
  method12Chirho :: a -> Int
  method13Chirho :: a -> Int
  method14Chirho :: a -> Int
  method15Chirho :: a -> Int
  method16Chirho :: a -> Int
  method17Chirho :: a -> Int
  method18Chirho :: a -> Int
  method19Chirho :: a -> Int
  method20Chirho :: a -> Int
  method21Chirho :: a -> Int
  method22Chirho :: a -> Int
  method23Chirho :: a -> Int
  method24Chirho :: a -> Int
  method25Chirho :: a -> Int
  method26Chirho :: a -> Int
  method27Chirho :: a -> Int
  method28Chirho :: a -> Int
  method29Chirho :: a -> Int
  method30Chirho :: a -> Int
  method31Chirho :: a -> Int
  method32Chirho :: a -> Int
  method33Chirho :: a -> Int
  method34Chirho :: a -> Int
  method35Chirho :: a -> Int
  method36Chirho :: a -> Int
  method37Chirho :: a -> Int
  method38Chirho :: a -> Int
  method39Chirho :: a -> Int
  method40Chirho :: a -> Int
  method41Chirho :: a -> Int
  method42Chirho :: a -> Int
  method43Chirho :: a -> Int
  method44Chirho :: a -> Int
  method45Chirho :: a -> Int
  method46Chirho :: a -> Int
  method47Chirho :: a -> Int
  method48Chirho :: a -> Int
  method49Chirho :: a -> Int
  method50Chirho :: a -> Int
  method51Chirho :: a -> Int
  method52Chirho :: a -> Int
  method53Chirho :: a -> Int
  method54Chirho :: a -> Int
  method55Chirho :: a -> Int
  method56Chirho :: a -> Int
  method57Chirho :: a -> Int
  method58Chirho :: a -> Int
  method59Chirho :: a -> Int
  method60Chirho :: a -> Int
  method61Chirho :: a -> Int
  method62Chirho :: a -> Int
  method63Chirho :: a -> Int
instance ManyChirho () where
  method0Chirho _ = 0
  method1Chirho _ = 1
  method2Chirho _ = 2
  method3Chirho _ = 3
  method4Chirho _ = 4
  method5Chirho _ = 5
  method6Chirho _ = 6
  method7Chirho _ = 7
  method8Chirho _ = 8
  method9Chirho _ = 9
  method10Chirho _ = 10
  method11Chirho _ = 11
  method12Chirho _ = 12
  method13Chirho _ = 13
  method14Chirho _ = 14
  method15Chirho _ = 15
  method16Chirho _ = 16
  method17Chirho _ = 17
  method18Chirho _ = 18
  method19Chirho _ = 19
  method20Chirho _ = 20
  method21Chirho _ = 21
  method22Chirho _ = 22
  method23Chirho _ = 23
  method24Chirho _ = 24
  method25Chirho _ = 25
  method26Chirho _ = 26
  method27Chirho _ = 27
  method28Chirho _ = 28
  method29Chirho _ = 29
  method30Chirho _ = 30
  method31Chirho _ = 31
  method32Chirho _ = 32
  method33Chirho _ = 33
  method34Chirho _ = 34
  method35Chirho _ = 35
  method36Chirho _ = 36
  method37Chirho _ = 37
  method38Chirho _ = 38
  method39Chirho _ = 39
  method40Chirho _ = 40
  method41Chirho _ = 41
  method42Chirho _ = 42
  method43Chirho _ = 43
  method44Chirho _ = 44
  method45Chirho _ = 45
  method46Chirho _ = 46
  method47Chirho _ = 47
  method48Chirho _ = 48
  method49Chirho _ = 49
  method50Chirho _ = 50
  method51Chirho _ = 51
  method52Chirho _ = 52
  method53Chirho _ = 53
  method54Chirho _ = 54
  method55Chirho _ = 55
  method56Chirho _ = 56
  method57Chirho _ = 57
  method58Chirho _ = 58
  method59Chirho _ = 59
  method60Chirho _ = 60
  method61Chirho _ = 61
  method62Chirho _ = 62
  method63Chirho _ = 63
"#;

fn frontend_chirho(
    source_chirho: &str,
    environment_chirho: &ClassEnvChirho,
) -> Result<crate::FrontendResultChirho, haskelujah_diagnostics_chirho::DiagnosticBundleChirho> {
    let mut map_chirho = SourceMapChirho::new_chirho();
    let file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut map_chirho,
        "ResourceControlChirho.hs",
        source_chirho,
    );
    run_frontend_with_type_synonyms_families_and_class_env_chirho(
        source_chirho,
        file_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &HashMap::new(),
        &ImportedTypeSynonymsChirho::new(),
        &seed_builtin_type_families_chirho(),
        environment_chirho,
    )
}

#[test]
fn registry_failure_is_fatal_under_real_and_comment_triggered_defer_chirho() {
    let mut failed_chirho = ClassEnvChirho::with_registration_limit_chirho(0);
    failed_chirho.seed_standard_chirho();
    assert!(failed_chirho.registry_error_chirho().is_some());
    for source_chirho in [REAL_DEFER_CHIRHO, COMMENT_DEFER_CHIRHO] {
        let error_chirho = match frontend_chirho(source_chirho, &failed_chirho) {
            Ok(_) => panic!("a resource failure cannot publish a seed-only frontend result"),
            Err(error_chirho) => error_chirho,
        };
        assert!(
            error_chirho
                .to_string()
                .contains("class registry resource limit reached")
        );
    }
}

#[test]
fn many_methods_use_actual_snapshot_restore_without_spending_the_module_allowance_chirho() {
    let mut imported_chirho = ClassEnvChirho::with_registration_limit_chirho(65536);
    imported_chirho.seed_standard_chirho();
    let result_chirho = frontend_chirho(MANY_METHODS_CHIRHO, &imported_chirho)
        .expect("every method is checked under its normal restore scope");
    assert!(
        !result_chirho
            .infer_result_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert!(
        !result_chirho
            .infer_result_chirho
            .diagnostics_chirho
            .has_errors_chirho()
    );
    assert!(
        result_chirho
            .infer_result_chirho
            .class_env_chirho
            .remaining_registration_work_chirho()
            > 0
    );
}

#[test]
fn a_module_chain_gets_fresh_module_work_after_each_complete_checked_import_chirho() {
    let mut imported_chirho = ClassEnvChirho::with_registration_limit_chirho(65536);
    imported_chirho.seed_standard_chirho();
    for index_chirho in 0..64 {
        let source_chirho = format!(
            "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Chain{index_chirho}Chirho where\nclass Class{index_chirho}Chirho a where\n  method{index_chirho}Chirho :: a -> a\n"
        );
        let result_chirho = frontend_chirho(&source_chirho, &imported_chirho)
            .expect("each complete next module is admitted");
        assert!(
            !result_chirho
                .infer_result_chirho
                .non_deferrable_diagnostics_chirho
                .has_errors_chirho()
        );
        assert!(
            !result_chirho
                .infer_result_chirho
                .diagnostics_chirho
                .has_errors_chirho()
        );
        imported_chirho = result_chirho
            .infer_result_chirho
            .class_env_chirho
            .try_snapshot_chirho()
            .expect("complete snapshot publishes no spent allowance into the next module");
        assert_eq!(imported_chirho.registration_limit_chirho(), 65536);
        assert!(imported_chirho.remaining_registration_work_chirho() > 0);
    }
}

#[test]
fn failed_admission_cannot_export_evidence_or_become_a_successful_next_import_chirho() {
    let healthy_chirho = frontend_chirho(REAL_DEFER_CHIRHO, &ClassEnvChirho::new_chirho())
        .expect("the source itself is valid");
    let mut failed_chirho = ClassEnvChirho::with_registration_limit_chirho(0);
    failed_chirho.seed_standard_chirho();
    let infer_chirho = |environment_chirho: &ClassEnvChirho| {
        haskelujah_typing_chirho::infer_chirho::infer_module_with_import_context_chirho(
            &healthy_chirho.module_chirho,
            haskelujah_typing_chirho::infer_chirho::InferenceImportContextChirho {
                unary_variable_instance_heads_covered_chirho: false,
                local_class_prefix_instances_complete_chirho: false,
                imported_types_chirho: &HashMap::new(),
                selected_imported_value_schemes_chirho: Vec::new(),
                imported_type_synonyms_chirho: &HashMap::new(),
                imported_type_families_chirho: &Default::default(),
                imported_class_env_chirho: environment_chirho,
                imported_record_field_names_chirho: &HashMap::new(),
                unavailable_values_chirho: &std::collections::HashSet::new(),
                safe_unqualified_imported_type_names_chirho: &Default::default(),
                preferred_qualified_type_names_chirho: &HashMap::new(),
                promotion_scope_chirho: None,
                family_scope_chirho: None,
                type_constructor_scope_chirho: None,
                entry_obligation_chirho: None,
            },
        )
    };
    let first_chirho = infer_chirho(&failed_chirho);
    assert!(
        first_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert!(
        first_chirho
            .class_env_chirho
            .registry_error_chirho()
            .is_some()
    );
    assert!(first_chirho.method_occurrences_chirho.is_empty());
    assert!(first_chirho.reference_evidence_chirho.is_empty());
    assert!(first_chirho.source_instances_chirho.is_empty());
    let next_chirho = infer_chirho(&first_chirho.class_env_chirho);
    assert!(
        next_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert!(
        next_chirho
            .class_env_chirho
            .registry_error_chirho()
            .is_some()
    );
    assert!(next_chirho.method_occurrences_chirho.is_empty());
    let backend_chirho = crate::compile_backend_chirho(
        healthy_chirho.module_chirho,
        next_chirho,
        Default::default(),
        &[],
        &Default::default(),
    );
    assert!(
        backend_chirho.is_err(),
        "backend admission independently rejects the fatal inference channel"
    );
}
