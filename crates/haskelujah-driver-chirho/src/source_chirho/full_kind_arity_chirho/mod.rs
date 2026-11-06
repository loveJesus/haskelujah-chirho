// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written full-kind GND parameters retain actual universe authority.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::language_options_tests_chirho::check_chirho;

mod class_headers_chirho;
mod quantifier_prefix_chirho;

#[test]
fn selected_qualified_universes_and_grammar_stars_reach_deriving_chirho() {
    for source_chirho in [
        include_str!("QualifiedUniverseChirho.hs"),
        include_str!("GrammarStarChirho.hs"),
    ] {
        if let Err(error_chirho) = check_chirho(source_chirho, &[]) {
            panic!(
                "a checked universe proves the written higher-kinded class parameter: {error_chirho}"
            );
        }
    }
}

#[test]
fn source_universe_homonyms_and_disabled_star_cannot_supply_kinds_chirho() {
    for source_chirho in [
        include_str!("LocalUniverseShadowChirho.hs"),
        include_str!("GrammarNoStarChirho.hs"),
    ] {
        let error_chirho = check_chirho(source_chirho, &[])
            .err()
            .expect("the written annotation has no lifted universe authority");
        assert!(
            error_chirho.to_string().contains("error[E"),
            "{error_chirho}"
        );
    }
}

fn has_derived_wrap_instance_chirho(module_chirho: &ModuleChirho) -> bool {
    module_chirho.decls_chirho.iter().any(|declaration_chirho| {
        matches!(
            declaration_chirho,
            DeclChirho::InstanceDeclChirho { class_chirho, types_chirho, .. }
                if class_chirho.text_chirho() == "MarkerChirho"
                    && matches!(
                        types_chirho.last(),
                        Some(TypeChirho::ConChirho(name_chirho))
                            if name_chirho.text_chirho() == "WrapChirho"
                    )
        )
    })
}

#[test]
fn a_proofless_parsed_universe_projection_cannot_create_an_eta_reduced_instance_chirho() {
    let source_chirho = include_str!("QualifiedUniverseChirho.hs");
    let syntax_chirho = haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .parse_chirho();
    let module_chirho = haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &syntax_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let interfaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let classes_chirho = haskelujah_naming_chirho::build_builtin_class_scope_chirho(
        &module_chirho,
        &interfaces_chirho,
    );
    let mut scope_chirho = haskelujah_naming_chirho::build_type_constructor_scope_chirho(
        &module_chirho,
        &interfaces_chirho,
    );
    let mut checked_chirho = haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &syntax_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let warnings_chirho =
        haskelujah_typing_chirho::deriving_chirho::apply_deriving_with_type_scope_chirho(
            &mut checked_chirho,
            &classes_chirho,
            Some(&scope_chirho),
        );
    assert!(warnings_chirho.is_empty(), "{warnings_chirho:?}");
    assert!(has_derived_wrap_instance_chirho(&checked_chirho));

    scope_chirho.bind_chirho("KindChirho.Type".to_string(), None);
    let mut proofless_chirho = module_chirho;
    let validity_chirho =
        haskelujah_typing_chirho::check_module_declaration_validity_with_type_scope_diagnostics_chirho(
            &proofless_chirho,
            &classes_chirho,
            Some(&scope_chirho),
        );
    assert!(validity_chirho.diagnostics_chirho.has_errors_chirho());
    let warnings_chirho =
        haskelujah_typing_chirho::deriving_chirho::apply_deriving_with_type_scope_chirho(
            &mut proofless_chirho,
            &classes_chirho,
            Some(&scope_chirho),
        );
    assert!(
        warnings_chirho.is_empty(),
        "unsupported written strategies are declaration errors"
    );
    assert!(!has_derived_wrap_instance_chirho(&proofless_chirho));
}
