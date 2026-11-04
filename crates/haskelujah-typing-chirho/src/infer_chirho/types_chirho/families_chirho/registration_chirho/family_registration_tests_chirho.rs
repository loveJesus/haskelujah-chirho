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
        kind_args_chirho: Vec::new(),
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
        data_family_chirho: false,
        standalone_kind_chirho: None,
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
                kind_args_chirho: Vec::new(),
                class_chirho: dummy_name_chirho("Eq"),
                args_chirho: vec![syntax_app_chirho(
                    "FChirho",
                    vec![syntax_app_chirho("Int", vec![])],
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(ConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
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
            default_arguments_chirho: None,
            name_chirho: dummy_name_chirho("RepChirho"),
            result_kind_chirho: None,
            type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
            data_family_chirho: false,
            default_rhs_chirho: default_chirho.then(|| syntax_app_chirho("Int", vec![])),
            default_params_chirho: vec![],
            default_binders_chirho: None,
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

#[test]
fn data_family_heads_are_generative_but_type_family_heads_remain_opaque_chirho() {
    // T17067's distinction: distinct DATA family heads are nominally apart.
    // The same syntax built with TYPE families supplies no such evidence.
    for data_family_chirho in [false, true] {
        let header_chirho = |name_chirho| DeclChirho::TypeFamilyDeclChirho {
            name_chirho: dummy_name_chirho(name_chirho),
            type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
            result_kind_chirho: None,
            data_family_chirho,
            standalone_kind_chirho: None,
            closed_chirho: false,
            equations_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho =
            module_chirho(vec![header_chirho("D1Chirho"), header_chirho("D2Chirho")]);
        let mut ctx_chirho = family_fixture_context_chirho();
        ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
        ctx_chirho.register_module_families_chirho(&module_chirho);
        ctx_chirho.register_open_family_chirho("FChirho".to_owned(), 1, SpanChirho::DUMMY_CHIRHO);
        for (name_chirho, rhs_chirho) in [
            ("D1Chirho", TyChirho::int_chirho()),
            ("D2Chirho", TyChirho::char_chirho()),
        ] {
            ctx_chirho.register_type_family_instance_chirho(
                "FChirho".to_owned(),
                vec![type_family_app1_chirho(
                    name_chirho,
                    TyChirho::ForallVarChirho("aChirho".to_owned()),
                )],
                rhs_chirho,
            );
        }
        assert_eq!(
            ctx_chirho.reduce_type_family_chirho(
                "FChirho",
                &[type_family_app1_chirho("D2Chirho", TyChirho::bool_chirho())]
            ),
            data_family_chirho.then(TyChirho::char_chirho)
        );
        assert_eq!(
            ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            !data_family_chirho
        );
    }
}

#[test]
fn source_d1_data_family_shadows_the_bare_generic_alias_only_chirho() {
    // Literal names are essential: T17067 collides with the compiler's D1 alias.
    let header_chirho = |name_chirho, data_family_chirho| DeclChirho::TypeFamilyDeclChirho {
        name_chirho: dummy_name_chirho(name_chirho),
        type_vars_chirho: if name_chirho == "D2" {
            vec![]
        } else {
            vec![dummy_name_chirho("aChirho").into()]
        },
        result_kind_chirho: (name_chirho == "D2").then(|| TypeChirho::FunChirho {
            arg_chirho: Box::new(syntax_app_chirho("Type", vec![])),
            mult_chirho: None,
            result_chirho: Box::new(syntax_app_chirho("Type", vec![])),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        standalone_kind_chirho: None,
        data_family_chirho,
        closed_chirho: false,
        equations_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let instance_chirho = |name_chirho, start_chirho| DeclChirho::TypeFamilyInstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        family_name_chirho: dummy_name_chirho("F"),
        lhs_types_chirho: vec![syntax_app_chirho(
            name_chirho,
            vec![syntax_var_chirho("aChirho")],
        )],
        rhs_chirho: syntax_var_chirho("aChirho"),
        span_chirho: SpanChirho::new_chirho(
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
            haskelujah_span_chirho::ByteOffsetChirho::new_chirho(start_chirho),
            haskelujah_span_chirho::ByteOffsetChirho::new_chirho(start_chirho + 10),
        ),
    };
    let module_chirho = module_chirho(vec![
        header_chirho("D1", true),
        header_chirho("D2", true),
        header_chirho("F", false),
        instance_chirho("D1", 10),
        instance_chirho("D2", 30),
    ]);
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
    ctx_chirho.register_module_families_chirho(&module_chirho);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{}",
        ctx_chirho.diagnostics_chirho
    );
    for (name_chirho, argument_chirho) in [
        ("D1", TyChirho::int_chirho()),
        ("D2", TyChirho::bool_chirho()),
    ] {
        assert_eq!(
            ctx_chirho.reduce_type_family_chirho(
                "F",
                &[type_family_app1_chirho(
                    name_chirho,
                    argument_chirho.clone()
                )]
            ),
            Some(argument_chirho)
        );
    }
    let local_chirho = type_family_app1_chirho("D1", TyChirho::int_chirho());
    assert_eq!(
        ctx_chirho.expand_type_synonyms_chirho(&local_chirho),
        local_chirho
    );
    assert_eq!(
        ctx_chirho.expand_type_synonyms_chirho(&type_family_app1_chirho(
            "GHC.Generics.D1",
            TyChirho::int_chirho()
        )),
        type_family_app2_chirho(
            "M1",
            TyChirho::ConChirho("D".to_owned()),
            TyChirho::int_chirho()
        )
    );
}

#[test]
fn generic_nominal_seeds_do_not_authorize_synonyms_or_source_collisions_chirho() {
    for name_chirho in ["Par1", "M1", "V1"] {
        let mut registry_chirho = seed_type_family_registry_chirho();
        assert!(matches!(
            super::family_registration_chirho::family_head_role_chirho(
                &registry_chirho,
                name_chirho
            ),
            super::closed_family_apartness_chirho::FamilyHeadRoleChirho::GenerativeChirho { .. }
        ));
        registry_chirho
            .constructor_origins_mut_chirho()
            .replace_source_module_chirho(
                "SourceCollisionChirho",
                &[(name_chirho.to_owned(), true)],
            )
            .unwrap();
        assert!(matches!(
            super::family_registration_chirho::family_head_role_chirho(
                &registry_chirho,
                name_chirho
            ),
            super::closed_family_apartness_chirho::FamilyHeadRoleChirho::UnknownChirho
        ));
    }
    let registry_chirho = seed_type_family_registry_chirho();
    for name_chirho in ["D1", "C1", "Rec0"] {
        assert!(
            registry_chirho
                .constructor_origins_chirho()
                .identity_chirho(name_chirho)
                .is_none()
        );
    }
}

#[test]
fn generic_rep_seed_is_open_and_enters_scope_only_by_its_export_chirho() {
    let registry_chirho = seed_type_family_registry_chirho();
    let id_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
        module_chirho: "GHC.Generics".to_owned(),
        name_chirho: "Rep".to_owned(),
    };
    let key_chirho = id_chirho.key_chirho();
    let definition_chirho = registry_chirho.definition_chirho(&key_chirho).unwrap();
    assert_eq!(definition_chirho.arity_chirho(), 1);
    assert_eq!(
        definition_chirho.reduction_chirho(),
        TypeFamilyReductionChirho::OpenChirho
    );
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    assert!(matches!(
        scope_chirho.resolve_chirho("Rep", &registry_chirho),
        TypeFamilyScopeResolutionChirho::MissingChirho
    ));
    scope_chirho.import_export_chirho("Rep".to_owned(), "GHC.Generics.Rep", &registry_chirho);
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.family_scope_chirho = Some(Arc::new(scope_chirho));
    ctx_chirho.register_type_family_instance_chirho(
        "Rep".to_owned(),
        vec![TyChirho::bool_chirho()],
        TyChirho::int_chirho(),
    );
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("Rep", &[TyChirho::bool_chirho()]),
        Some(TyChirho::int_chirho())
    );
    assert_eq!(
        ctx_chirho.family_name_key_chirho("Rep"),
        Some(key_chirho.clone())
    );
    ctx_chirho.register_open_family_chirho("Rep".to_owned(), 1, SpanChirho::DUMMY_CHIRHO);
    assert_ne!(ctx_chirho.family_name_key_chirho("Rep"), Some(key_chirho));
}

fn nominal_class_chirho(
    name_chirho: &str,
    associated_tfs_chirho: Vec<haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho>,
) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: dummy_name_chirho(name_chirho),
        type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
        methods_chirho: vec![],
        associated_tfs_chirho,
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn source_class_valued_associated_rhs_reduces_and_reapplies_extra_arguments_chirho() {
    // T5655's `type C ETwice = Twice`: the result is a real class constructor,
    // not an arbitrary raw name granted Result-mode authority.
    let module_chirho = module_chirho(vec![
        nominal_class_chirho("Twice", vec![]),
        DeclChirho::DataDeclChirho {
            name_chirho: dummy_name_chirho("ETwice"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        nominal_class_chirho(
            "E",
            vec![haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                default_arguments_chirho: None,
                name_chirho: dummy_name_chirho("C"),
                result_kind_chirho: None,
                type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
                data_family_chirho: false,
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                default_binders_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        ),
    ]);
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
    ctx_chirho.register_module_families_chirho(&module_chirho);
    ctx_chirho.register_family_instance_ast_chirho(
        "C".to_owned(),
        &[syntax_app_chirho("ETwice", vec![])],
        &syntax_app_chirho("Twice", vec![]),
        SpanChirho::DUMMY_CHIRHO,
        Some(0),
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{}",
        ctx_chirho.diagnostics_chirho
    );
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("C", &[TyChirho::ConChirho("ETwice".to_owned())]),
        Some(TyChirho::ConChirho("Twice".to_owned()))
    );
    assert_eq!(
        ctx_chirho.reduce_type_family_chirho(
            "C",
            &[
                TyChirho::ConChirho("ETwice".to_owned()),
                TyChirho::int_chirho()
            ]
        ),
        Some(type_family_app1_chirho("Twice", TyChirho::int_chirho()))
    );
}

#[test]
fn source_classes_are_nominally_apart_but_synonyms_and_collisions_stay_unknown_chirho() {
    use super::closed_family_apartness_chirho::{
        FamilyHeadRoleChirho, FamilyUnificationChirho, family_patterns_unification_chirho,
    };
    let source_module_chirho = module_chirho(vec![
        nominal_class_chirho("FirstClassChirho", vec![]),
        nominal_class_chirho("SecondClassChirho", vec![]),
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: dummy_name_chirho("ClassAliasChirho"),
            type_vars_chirho: vec![],
            rhs_chirho: syntax_app_chirho("FirstClassChirho", vec![]),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);
    let mut ctx_chirho = family_fixture_context_chirho();
    ctx_chirho.prepare_family_scope_chirho(&source_module_chirho, None);
    let classify_chirho = |name_chirho: &str| {
        super::family_registration_chirho::family_head_role_chirho(
            &ctx_chirho.type_families_chirho,
            name_chirho,
        )
    };
    assert_eq!(
        family_patterns_unification_chirho(
            &[type_family_app1_chirho(
                "FirstClassChirho",
                TyChirho::int_chirho()
            )],
            &[type_family_app1_chirho(
                "SecondClassChirho",
                TyChirho::int_chirho()
            )],
            &classify_chirho
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
    assert!(matches!(
        classify_chirho("ClassAliasChirho"),
        FamilyHeadRoleChirho::UnknownChirho
    ));
    let mut collision_chirho =
        module_chirho(vec![nominal_class_chirho("FirstClassChirho", vec![])]);
    collision_chirho.name_chirho = dummy_name_chirho("OtherClassModuleChirho");
    ctx_chirho.prepare_family_scope_chirho(&collision_chirho, None);
    assert!(matches!(
        super::family_registration_chirho::family_head_role_chirho(
            &ctx_chirho.type_families_chirho,
            "FirstClassChirho"
        ),
        FamilyHeadRoleChirho::UnknownChirho
    ));
}
