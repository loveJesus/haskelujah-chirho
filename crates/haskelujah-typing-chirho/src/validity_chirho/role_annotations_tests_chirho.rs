// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 17 to 19 on root's measured role-annotation shapes (`role-annotation-r1-chirho`,
//! GHC 9.14.1): RA1, a written phantom over a stored parameter, is GHC-29178; RA2, two roles
//! for one parameter, GHC-54298; RA3, the word `representation`, GHC-09009; RA4, a written
//! representational role, holds. A role the module cannot prove is a named gap.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, ConPrefixChirho, DeclChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use super::*;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

/// `data <type> <parameters> = <type> <fields>`.
fn data_chirho(
    type_chirho: &str,
    parameters_chirho: &[&str],
    fields_chirho: Vec<TypeChirho>,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_chirho),
        type_vars_chirho: parameters_chirho
            .iter()
            .map(|parameter_chirho| TyVarChirho::plain_chirho(name_chirho(parameter_chirho)))
            .collect(),
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: name_chirho(type_chirho),
            fields_chirho: fields_chirho
                .into_iter()
                .map(|field_chirho| (StrictnessChirho::LazyChirho, field_chirho))
                .collect(),
            prefix_chirho: ConPrefixChirho::default(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `type role <type> <words>`.
fn role_chirho(type_chirho: &str, words_chirho: &[&str]) -> DeclChirho {
    DeclChirho::RoleAnnotationDeclChirho {
        type_name_chirho: name_chirho(type_chirho),
        roles_chirho: words_chirho
            .iter()
            .map(|word_chirho| name_chirho(word_chirho))
            .collect(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec!["RoleAnnotations".to_string()],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

/// The codes rules 17 to 19 report for a module of these declarations.
fn codes_chirho(decls_chirho: Vec<DeclChirho>) -> Vec<&'static str> {
    let mut errors_chirho = Vec::new();
    check_role_annotations_chirho(&module_chirho(decls_chirho), &mut errors_chirho);
    errors_chirho
        .iter()
        .map(|error_chirho| error_chirho.ghc_code_chirho)
        .collect()
}

/// R3's `HoldChirho`, whose stored parameter is representational.
fn hold_chirho() -> DeclChirho {
    data_chirho("HoldChirho", &["aChirho"], vec![var_chirho("aChirho")])
}

#[test]
fn a_written_role_below_the_final_role_is_ghcs_mismatch_chirho() {
    assert_eq!(
        codes_chirho(vec![role_chirho("HoldChirho", &["phantom"]), hold_chirho()]),
        ["GHC-29178"]
    );
    for holding_chirho in ["representational", "nominal", "_"] {
        assert!(
            codes_chirho(vec![
                role_chirho("HoldChirho", &[holding_chirho]),
                hold_chirho()
            ])
            .is_empty(),
            "{holding_chirho}"
        );
    }
}

#[test]
fn a_wrong_count_and_an_unknown_word_are_refused_as_written_chirho() {
    assert_eq!(
        codes_chirho(vec![
            role_chirho("HoldChirho", &["nominal", "nominal"]),
            hold_chirho()
        ]),
        ["GHC-54298"]
    );
    assert_eq!(
        codes_chirho(vec![
            role_chirho("HoldChirho", &["representation"]),
            hold_chirho()
        ]),
        ["GHC-09009"]
    );
    // A type without parameters takes no role: the old eval_data fixture's shape.
    assert_eq!(
        codes_chirho(vec![
            data_chirho("TagChirho", &[], vec![]),
            role_chirho("TagChirho", &["nominal"])
        ]),
        ["GHC-54298"]
    );
}

#[test]
fn an_unknown_final_role_is_a_named_gap_not_ghcs_mismatch_chirho() {
    // `HeldChirho (Maybe aChirho)`: Maybe's roles are not the module's to prove.
    let held_chirho = data_chirho(
        "HeldChirho",
        &["aChirho"],
        vec![TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(name_chirho("Maybe"))),
            arg_chirho: Box::new(var_chirho("aChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );
    assert_eq!(
        codes_chirho(vec![
            role_chirho("HeldChirho", &["representational"]),
            held_chirho.clone()
        ]),
        [UNSUPPORTED_SHAPE_CHIRHO]
    );
    assert!(codes_chirho(vec![role_chirho("HeldChirho", &["nominal"]), held_chirho]).is_empty());
}
