// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 17 to 24 on root's measured role-annotation shapes (GHC 9.14.1):
//! `role-annotation-r1-chirho`'s RA1, a written phantom over a stored parameter, is
//! GHC-29178; RA2, two roles for one parameter, GHC-54298; RA3, the word `representation`,
//! GHC-09009; RA4, a written representational role, holds. `role-surfaces-and-direct-gnd-r1-chirho`'s
//! RX1 and RX3 (no RoleAnnotations) are GHC-17779, RX4 (a synonym) GHC-77192, RX6 (a
//! representational class parameter) GHC-18273, RX7 and RX8 (an undeclared and an imported
//! target) GHC-44432 and RX9 (a repeat) GHC-97170; RX5's nominal class parameter holds. A role
//! the module cannot prove is a named gap.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, ClassMethodChirho, ConDeclChirho, ConPrefixChirho, DeclChirho,
    StrictnessChirho, TyVarChirho,
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
    module_with_chirho(decls_chirho, &["RoleAnnotations"])
}

fn module_with_chirho(decls_chirho: Vec<DeclChirho>, extensions_chirho: &[&str]) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: extensions_chirho
            .iter()
            .map(|extension_chirho| extension_chirho.to_string())
            .collect(),
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

/// The codes rules 17 to 24 report for a module of these declarations.
fn codes_chirho(decls_chirho: Vec<DeclChirho>) -> Vec<&'static str> {
    codes_in_chirho(module_chirho(decls_chirho))
}

fn codes_in_chirho(module_chirho: ModuleChirho) -> Vec<&'static str> {
    let mut errors_chirho = Vec::new();
    check_role_annotations_chirho(&module_chirho, &mut errors_chirho);
    errors_chirho
        .iter()
        .map(|error_chirho| error_chirho.ghc_code_chirho)
        .collect()
}

fn app_chirho(function_chirho: &str, argument_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::ConChirho(name_chirho(function_chirho))),
        arg_chirho: Box::new(argument_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `class <class> aChirho where wrapChirho :: aChirho -> <result>`, with these associated
/// families of `aChirho`.
fn class_chirho(
    class_name_chirho: &str,
    result_chirho: TypeChirho,
    families_chirho: &[&str],
) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho(class_name_chirho),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("aChirho"))],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: name_chirho("wrapChirho"),
            ty_chirho: TypeChirho::FunChirho {
                arg_chirho: Box::new(var_chirho("aChirho")),
                mult_chirho: None,
                result_chirho: Box::new(result_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: families_chirho
            .iter()
            .map(|family_chirho| AssocTypeFamilyChirho {
                name_chirho: name_chirho(family_chirho),
                type_vars_chirho: vec![name_chirho("aChirho")],
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect(),
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// RX5 and RX6's `WrapChirho`, whose method returns `HoldChirho aChirho`.
fn wrap_chirho() -> DeclChirho {
    class_chirho(
        "WrapChirho",
        app_chirho("HoldChirho", var_chirho("aChirho")),
        &[],
    )
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

#[test]
fn an_annotation_without_role_annotations_is_ghcs_17779_and_checking_goes_on_chirho() {
    // RX1: RA4 without the extension.
    assert_eq!(
        codes_in_chirho(module_with_chirho(
            vec![
                role_chirho("HoldChirho", &["representational"]),
                hold_chirho()
            ],
            &["DerivingStrategies"]
        )),
        ["GHC-17779"]
    );
    assert_eq!(
        codes_in_chirho(module_with_chirho(
            vec![role_chirho("HoldChirho", &["phantom"]), hold_chirho()],
            &[]
        )),
        ["GHC-17779", "GHC-29178"]
    );
    assert!(
        codes_in_chirho(module_with_chirho(
            vec![
                role_chirho("HoldChirho", &["representational"]),
                hold_chirho()
            ],
            &["RoleAnnotations", "NoRoleAnnotations", "RoleAnnotations"]
        ))
        .is_empty()
    );
}

#[test]
fn a_synonym_or_family_takes_no_roles_chirho() {
    // RX4: GHC-77192, with or without the extension.
    let synonym_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("HeldChirho"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("aChirho"))],
        rhs_chirho: app_chirho("HoldChirho", var_chirho("aChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let annotated_chirho = vec![
        role_chirho("HeldChirho", &["representational"]),
        hold_chirho(),
        synonym_chirho,
    ];
    assert_eq!(codes_chirho(annotated_chirho.clone()), ["GHC-77192"]);
    assert_eq!(
        codes_in_chirho(module_with_chirho(annotated_chirho, &[])),
        ["GHC-77192"]
    );
    let family_chirho = DeclChirho::TypeFamilyDeclChirho {
        name_chirho: name_chirho("FamChirho"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("aChirho"))],
        result_kind_chirho: None,
        equations_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        codes_chirho(vec![role_chirho("FamChirho", &["nominal"]), family_chirho]),
        ["GHC-77192"]
    );
    // An associated family's check is unmeasured.
    assert_eq!(
        codes_chirho(vec![
            role_chirho("AssocChirho", &["nominal"]),
            hold_chirho(),
            class_chirho("OwnerChirho", var_chirho("aChirho"), &["AssocChirho"])
        ]),
        [UNSUPPORTED_SHAPE_CHIRHO]
    );
}

#[test]
fn a_class_parameter_is_nominal_unless_incoherent_instances_chirho() {
    // RX5 holds; RX6 is GHC-18273, its final role representational as written.
    assert!(
        codes_chirho(vec![
            role_chirho("WrapChirho", &["nominal"]),
            hold_chirho(),
            wrap_chirho()
        ])
        .is_empty()
    );
    assert!(
        codes_chirho(vec![
            role_chirho("WrapChirho", &["_"]),
            hold_chirho(),
            wrap_chirho()
        ])
        .is_empty()
    );
    let representational_chirho = vec![
        role_chirho("WrapChirho", &["representational"]),
        hold_chirho(),
        wrap_chirho(),
    ];
    assert_eq!(codes_chirho(representational_chirho.clone()), ["GHC-18273"]);
    assert!(
        codes_in_chirho(module_with_chirho(
            representational_chirho,
            &["RoleAnnotations", "IncoherentInstances"]
        ))
        .is_empty()
    );
    // A written phantom below the method's representational use is also GHC-29178.
    assert_eq!(
        codes_chirho(vec![
            role_chirho("WrapChirho", &["phantom"]),
            hold_chirho(),
            wrap_chirho()
        ]),
        ["GHC-29178", "GHC-18273"]
    );
    // An associated family's parameter is nominal: the written role is below it.
    assert_eq!(
        codes_chirho(vec![
            role_chirho("OwnerChirho", &["representational"]),
            class_chirho("OwnerChirho", var_chirho("aChirho"), &["AssocChirho"])
        ]),
        ["GHC-29178"]
    );
    // Maybe's roles are not the module's to prove.
    assert_eq!(
        codes_chirho(vec![
            role_chirho("MaybeWrapChirho", &["representational"]),
            class_chirho(
                "MaybeWrapChirho",
                app_chirho("Maybe", var_chirho("aChirho")),
                &[]
            )
        ]),
        [UNSUPPORTED_SHAPE_CHIRHO]
    );
}

#[test]
fn a_target_the_module_does_not_declare_is_ghcs_44432_chirho() {
    // RX7, an undeclared target, and RX8, an imported one.
    for target_chirho in ["MissingChirho", "Maybe"] {
        assert_eq!(
            codes_chirho(vec![
                role_chirho(target_chirho, &["representational"]),
                hold_chirho()
            ]),
            ["GHC-44432"],
            "{target_chirho}"
        );
    }
    let qualified_chirho = DeclChirho::RoleAnnotationDeclChirho {
        type_name_chirho: NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            "Main",
            "HoldChirho",
            SpanChirho::DUMMY_CHIRHO,
        )),
        roles_chirho: vec![name_chirho("representational")],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        codes_chirho(vec![qualified_chirho, hold_chirho()]),
        [UNSUPPORTED_SHAPE_CHIRHO]
    );
}

#[test]
fn renamer_and_parse_errors_stop_the_later_stages_chirho() {
    // RX9: the repeat alone, though the module lacks RoleAnnotations.
    let repeated_chirho = vec![
        role_chirho("HoldChirho", &["representational"]),
        role_chirho("HoldChirho", &["representational"]),
        hold_chirho(),
    ];
    assert_eq!(codes_chirho(repeated_chirho.clone()), ["GHC-97170"]);
    assert_eq!(
        codes_in_chirho(module_with_chirho(repeated_chirho, &[])),
        ["GHC-97170"]
    );
    // Both renamer errors, and no role is judged.
    assert_eq!(
        codes_chirho(vec![
            role_chirho("HoldChirho", &["phantom"]),
            role_chirho("HoldChirho", &["phantom"]),
            role_chirho("MissingChirho", &["phantom"]),
            hold_chirho()
        ]),
        ["GHC-97170", "GHC-44432"]
    );
    // An unknown word is GHC's parse error: nothing else is reported.
    assert_eq!(
        codes_chirho(vec![
            role_chirho("HoldChirho", &["representation"]),
            role_chirho("HoldChirho", &["representational"]),
            role_chirho("MissingChirho", &["phantom"]),
            hold_chirho()
        ]),
        ["GHC-09009"]
    );
}
