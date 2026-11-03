// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Registration and transport contracts, separate from the pure reducer controls.

use super::family_definitions_chirho::{
    TypeFamilyBranchChirho, TypeFamilyBranchIdChirho, TypeFamilyDefinitionChirho,
    TypeFamilyReductionChirho,
};
use super::test_support_chirho::{
    dummy_name_chirho, family_fixture_context_chirho, type_family_app1_chirho,
    type_family_app2_chirho,
};
use super::*;
use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TypeFamilyEquationChirho};
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

fn syntax_app_chirho(name_chirho: &str, arguments_chirho: Vec<TypeChirho>) -> TypeChirho {
    arguments_chirho.into_iter().fold(
        TypeChirho::ConChirho(dummy_name_chirho(name_chirho)),
        |fun_chirho, arg_chirho| TypeChirho::AppChirho {
            fun_chirho: Box::new(fun_chirho),
            arg_chirho: Box::new(arg_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    )
}

fn syntax_var_chirho(name_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(dummy_name_chirho(name_chirho))
}

fn equation_chirho(
    name_chirho: &str,
    lhs_types_chirho: Vec<TypeChirho>,
    rhs_chirho: TypeChirho,
) -> TypeFamilyEquationChirho {
    TypeFamilyEquationChirho {
        head_chirho: Some(dummy_name_chirho(name_chirho)),
        explicit_binders_chirho: None,
        unwritten_binders_chirho: vec![],
        lhs_types_chirho,
        rhs_chirho,
        malformed_chirho: false,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn family_decl_chirho(
    name_chirho: &str,
    parameters_chirho: &[&str],
    equations_chirho: Vec<TypeFamilyEquationChirho>,
) -> DeclChirho {
    DeclChirho::TypeFamilyDeclChirho {
        name_chirho: dummy_name_chirho(name_chirho),
        type_vars_chirho: parameters_chirho
            .iter()
            .map(|name_chirho| dummy_name_chirho(name_chirho).into())
            .collect(),
        result_kind_chirho: None,
        closed_chirho: true,
        equations_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: dummy_name_chirho("RegistrationChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

#[test]
fn later_and_self_family_heads_keep_compatible_fallbacks_chirho() {
    // These are the compatibility shapes in Claude2's V01u/V01cu receipts.
    // No family body or mutable closed placeholder is installed for the prepass.
    for self_chirho in [false, true] {
        let rhs_name_chirho = if self_chirho { "FChirho" } else { "GChirho" };
        let rhs_chirho = |variable_chirho| {
            syntax_app_chirho(
                rhs_name_chirho,
                if self_chirho {
                    vec![
                        syntax_var_chirho(variable_chirho),
                        syntax_var_chirho(variable_chirho),
                    ]
                } else {
                    vec![syntax_var_chirho(variable_chirho)]
                },
            )
        };
        let mut declarations_chirho = vec![family_decl_chirho(
            "FChirho",
            &["aChirho", "bChirho"],
            vec![
                equation_chirho(
                    "FChirho",
                    vec![
                        syntax_app_chirho("Int", vec![]),
                        syntax_var_chirho("bChirho"),
                    ],
                    rhs_chirho("bChirho"),
                ),
                equation_chirho(
                    "FChirho",
                    vec![
                        syntax_var_chirho("aChirho"),
                        syntax_app_chirho("Int", vec![]),
                    ],
                    rhs_chirho("aChirho"),
                ),
            ],
        )];
        if !self_chirho {
            declarations_chirho.push(family_decl_chirho(
                "GChirho",
                &["aChirho"],
                vec![equation_chirho(
                    "GChirho",
                    vec![syntax_var_chirho("aChirho")],
                    syntax_app_chirho("Bool", vec![]),
                )],
            ));
        }
        let module_chirho = module_chirho(declarations_chirho);
        let mut ctx_chirho = family_fixture_context_chirho();
        ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
        ctx_chirho.register_module_families_chirho(&module_chirho);
        let variable_chirho = TyChirho::VarChirho(TyVarChirho(73));
        let key_chirho = ctx_chirho.family_name_key_chirho(rhs_name_chirho).unwrap();
        let expected_chirho = if self_chirho {
            type_family_app2_chirho(
                &key_chirho,
                variable_chirho.clone(),
                variable_chirho.clone(),
            )
        } else {
            type_family_app1_chirho(&key_chirho, variable_chirho.clone())
        };
        assert_eq!(
            ctx_chirho
                .reduce_type_family_chirho("FChirho", &[variable_chirho, TyChirho::int_chirho()]),
            Some(expected_chirho)
        );
        if !self_chirho {
            assert_eq!(
                ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
                    "FChirho",
                    TyChirho::VarChirho(TyVarChirho(73)),
                    TyChirho::int_chirho()
                )),
                TyChirho::bool_chirho()
            );
        }
    }
}

fn imported_open_row_chirho(
    producer_chirho: &str,
    input_chirho: TyChirho,
    output_chirho: TyChirho,
) -> TypeFamilyDefinitionChirho {
    let id_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: "OriginChirho".to_owned(),
        name_chirho: "OpenChirho".to_owned(),
    };
    TypeFamilyDefinitionChirho::new_chirho(
        id_chirho.clone(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![TypeFamilyBranchChirho {
            id_chirho: TypeFamilyBranchIdChirho {
                family_chirho: id_chirho,
                producer_module_chirho: producer_chirho.to_owned(),
                declaration_start_chirho: 0,
                equation_index_chirho: 0,
            },
            binders_chirho: vec![],
            lhs_chirho: vec![input_chirho],
            rhs_chirho: output_chirho,
            blockers_chirho: vec![],
            held_chirho: false,
        }],
    )
    .unwrap()
    .verify_open_coherence_chirho(&|_, _| true, 100)
    .unwrap()
}

#[test]
fn independent_import_unions_get_one_cold_coherence_check_chirho() {
    for conflicting_chirho in [false, true] {
        let mut ctx_chirho = family_fixture_context_chirho();
        let first_chirho = imported_open_row_chirho(
            "LeftChirho",
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
        );
        let key_chirho = first_chirho.id_chirho().key_chirho();
        ctx_chirho
            .type_families_chirho
            .insert_chirho(first_chirho)
            .unwrap();
        let mut incoming_chirho = TypeFamilyRegistryChirho::new_chirho();
        incoming_chirho
            .insert_chirho(imported_open_row_chirho(
                "RightChirho",
                if conflicting_chirho {
                    TyChirho::int_chirho()
                } else {
                    TyChirho::char_chirho()
                },
                TyChirho::char_chirho(),
            ))
            .unwrap();
        ctx_chirho
            .type_families_chirho
            .merge_chirho(&incoming_chirho)
            .unwrap();
        assert!(
            !ctx_chirho
                .type_families_chirho
                .definition_chirho(&key_chirho)
                .unwrap()
                .open_coherence_verified_chirho()
        );
        ctx_chirho.verify_imported_open_families_chirho();
        assert_eq!(
            ctx_chirho
                .type_families_chirho
                .definition_chirho(&key_chirho)
                .unwrap()
                .open_coherence_verified_chirho(),
            !conflicting_chirho
        );
        assert_eq!(
            ctx_chirho.reduce_type_family_chirho(&key_chirho, &[TyChirho::int_chirho()]),
            (!conflicting_chirho).then(TyChirho::bool_chirho)
        );
    }
}

#[test]
fn associated_ast_wildcards_are_distinct_and_named_variables_share_chirho() {
    for wildcard_chirho in [false, true] {
        let mut ctx_chirho = family_fixture_context_chirho();
        ctx_chirho.register_open_family_chirho(
            "PairChirho".to_owned(),
            2,
            SpanChirho::DUMMY_CHIRHO,
        );
        let pattern_chirho = || {
            if wildcard_chirho {
                TypeChirho::WildcardChirho {
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }
            } else {
                syntax_var_chirho("aChirho")
            }
        };
        ctx_chirho.register_family_instance_ast_chirho(
            "PairChirho".to_owned(),
            &[pattern_chirho(), pattern_chirho()],
            &syntax_app_chirho("Char", vec![]),
            SpanChirho::DUMMY_CHIRHO,
            Some(0),
        );
        assert_eq!(
            ctx_chirho.reduce_type_family_chirho(
                "PairChirho",
                &[TyChirho::int_chirho(), TyChirho::bool_chirho()]
            ),
            wildcard_chirho.then(TyChirho::char_chirho)
        );
        assert_eq!(
            ctx_chirho.reduce_type_family_chirho(
                "PairChirho",
                &[TyChirho::int_chirho(), TyChirho::int_chirho()]
            ),
            Some(TyChirho::char_chirho())
        );
    }
}

#[test]
fn unsupported_open_rhs_keeps_the_whole_union_held_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_open_family_chirho("OpenChirho".to_owned(), 1, SpanChirho::DUMMY_CHIRHO);
    let rhs_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![dummy_name_chirho("aChirho").into()],
        body_chirho: Box::new(syntax_var_chirho("aChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    ctx_chirho.register_family_instance_ast_chirho(
        "OpenChirho".to_owned(),
        &[syntax_app_chirho("Int", vec![])],
        &rhs_chirho,
        SpanChirho::DUMMY_CHIRHO,
        Some(0),
    );
    let key_chirho = ctx_chirho.family_name_key_chirho("OpenChirho").unwrap();
    let definition_chirho = ctx_chirho
        .type_families_chirho
        .definition_chirho(&key_chirho)
        .unwrap();
    assert!(definition_chirho.branches_chirho()[0].held_chirho);
    assert!(!definition_chirho.open_coherence_verified_chirho());
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("OpenChirho", &[TyChirho::int_chirho()]),
        None
    );
}

#[test]
fn qualified_constraint_arguments_are_stamped_without_rebinding_imports_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_open_family_chirho("FChirho".to_owned(), 1, SpanChirho::DUMMY_CHIRHO);
    let key_chirho = ctx_chirho.family_name_key_chirho("FChirho").unwrap();
    let mut preserved_chirho = dummy_name_chirho("FChirho");
    preserved_chirho.set_family_key_chirho("%family:unresolved:producer-chirho".to_owned());
    let mut ty_chirho = TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::QuantifiedChirho {
            vars_chirho: vec![],
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: dummy_name_chirho("Eq"),
                args_chirho: vec![syntax_app_chirho(
                    "FChirho",
                    vec![syntax_app_chirho("Int", vec![])],
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(ConstraintChirho::ClassChirho {
                class_chirho: dummy_name_chirho("Show"),
                args_chirho: vec![TypeChirho::ConChirho(preserved_chirho)],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(syntax_app_chirho("Bool", vec![])),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    stamp_family_references_chirho(
        &mut ty_chirho,
        ctx_chirho.family_scope_chirho.as_deref().unwrap(),
        &ctx_chirho.type_families_chirho,
        "ConsumerChirho",
    );
    let TypeChirho::QualChirho { context_chirho, .. } = ty_chirho else {
        panic!("qualified type retained")
    };
    let ConstraintChirho::QuantifiedChirho {
        context_chirho,
        body_chirho,
        ..
    } = &context_chirho[0]
    else {
        panic!("quantified constraint retained")
    };
    let ConstraintChirho::ClassChirho { args_chirho, .. } = &context_chirho[0] else {
        panic!("class constraint retained")
    };
    let TypeChirho::AppChirho { fun_chirho, .. } = &args_chirho[0] else {
        panic!("family application retained")
    };
    let TypeChirho::ConChirho(name_chirho) = fun_chirho.as_ref() else {
        panic!("family head retained")
    };
    assert_eq!(name_chirho.family_key_chirho(), Some(key_chirho.as_str()));
    let ConstraintChirho::ClassChirho { args_chirho, .. } = body_chirho.as_ref() else {
        panic!("class body retained")
    };
    let TypeChirho::ConChirho(name_chirho) = &args_chirho[0] else {
        panic!("imported head retained")
    };
    assert_eq!(
        name_chirho.family_key_chirho(),
        Some("%family:unresolved:producer-chirho")
    );
}

#[test]
fn nonempty_compiler_seed_is_generative_until_a_source_collision_chirho() {
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.register_open_family_chirho("SizeChirho".to_owned(), 1, SpanChirho::DUMMY_CHIRHO);
    ctx_chirho.register_type_family_instance_chirho(
        "SizeChirho".to_owned(),
        vec![type_family_app1_chirho(
            "NonEmpty",
            TyChirho::ForallVarChirho("aChirho".to_owned()),
        )],
        TyChirho::int_chirho(),
    );
    ctx_chirho.register_type_family_instance_chirho(
        "SizeChirho".to_owned(),
        vec![TyChirho::bool_chirho()],
        TyChirho::char_chirho(),
    );
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("SizeChirho", &[TyChirho::bool_chirho()]),
        Some(TyChirho::char_chirho())
    );
    ctx_chirho
        .type_families_chirho
        .constructor_origins_mut_chirho()
        .replace_source_module_chirho("UserChirho", &[("NonEmpty".to_owned(), true)])
        .unwrap();
    assert!(
        ctx_chirho
            .type_families_chirho
            .constructor_origins_chirho()
            .identity_chirho("NonEmpty")
            .is_none()
    );
}

#[test]
fn associated_header_and_default_supply_one_catalogue_declaration_chirho() {
    let associated_chirho =
        |default_chirho: bool| haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
            name_chirho: dummy_name_chirho("RepChirho"),
            type_vars_chirho: vec![dummy_name_chirho("aChirho")],
            default_rhs_chirho: default_chirho.then(|| syntax_app_chirho("Int", vec![])),
            default_params_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    for (shapes_chirho, errors_chirho) in [
        (vec![false, true], false),
        (vec![true, false], false),
        (vec![false, false], true),
        (vec![true, true], true),
        (vec![false, true, true], true),
    ] {
        let module_chirho = module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("HasRepChirho"),
            type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
            methods_chirho: vec![],
            associated_tfs_chirho: shapes_chirho.into_iter().map(associated_chirho).collect(),
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        let mut ctx_chirho = family_fixture_context_chirho();
        ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
        assert_eq!(
            ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            errors_chirho
        );
    }
}

#[test]
fn two_genuine_data_declarations_are_not_catalogue_deduplicated_chirho() {
    let declaration_chirho = DeclChirho::DataDeclChirho {
        name_chirho: dummy_name_chirho("DuplicateChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = module_chirho(vec![declaration_chirho.clone(), declaration_chirho]);
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}
