// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-level functional-dependency context closure regressions.
//! workflow: language-features-chirho/fundep-generalization-chirho

use std::collections::HashMap;

fn check_original_chirho(source_chirho: &str) -> crate::FrontendResultChirho {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let types_chirho = HashMap::new();
    let synonyms_chirho = crate::ImportedTypeSynonymsChirho::new();
    let families_chirho = crate::ImportedTypeFamiliesChirho::new_chirho();
    let classes_chirho = haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
    let check_chirho = |options_chirho: &crate::FrontendOptionsChirho| {
        crate::run_standalone_frontend_chirho(
            source_chirho,
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
            crate::FrontendImportsChirho {
                ifaces_chirho: &ifaces_chirho,
                types_chirho: &types_chirho,
                type_synonyms_chirho: &synonyms_chirho,
                type_families_chirho: &families_chirho,
                class_env_chirho: &classes_chirho,
            },
            options_chirho,
        )
    };
    check_chirho(&crate::FrontendOptionsChirho::default())
        .expect("the original functional-dependency program checks without deferral")
}

#[test]
fn nested_insertion_closes_its_functional_dependency_context_chirho() {
    check_original_chirho(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../ghc-tests-chirho/typecheck-chirho/should_compile/tc177.hs"
    )));
}

#[test]
fn determined_arguments_are_solved_jointly_with_the_instance_context_chirho() {
    check_original_chirho(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../ghc-tests-chirho/typecheck-chirho/should_compile/tc180.hs"
    )));
}

#[test]
fn a_tuple_created_only_by_value_syntax_has_checked_kind_metadata_chirho() {
    let result_chirho = check_original_chirho(
        "{-# LANGUAGE FunctionalDependencies, MultiParamTypeClasses, FlexibleInstances #-}\n\
         module ValueOnlyTupleChirho where\n\
         class ClassifyChirho aChirho bChirho | aChirho -> bChirho where\n\
           classifyChirho :: aChirho -> bChirho\n\
         instance ClassifyChirho aChirho Bool where\n\
           classifyChirho _ = undefined\n\
         probeChirho = classifyChirho (True, False)\n",
    );
    let tuple_chirho = result_chirho
        .infer_result_chirho
        .type_families_chirho
        .declaration_schemes_chirho()
        .get_chirho("(,)")
        .expect("a value-only tuple has the compiler's checked declaration");
    assert_eq!(tuple_chirho.declaration_chirho,
        Some(haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho::compiler_data_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho::TupleChirho(2),
        )));
}
