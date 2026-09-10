// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    AssocTfInstanceChirho, AssocTypeFamilyChirho, FieldDeclChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
};
use haskelujah_ast_chirho::name_chirho::RawNameChirho;
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho};

use crate::iface_chirho::builtin_module_ifaces_chirho;
use crate::resolve_chirho::{resolve_module_chirho, resolve_module_with_imports_chirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn qualified_name_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn tyvar_chirho(text_chirho: &str) -> TyVarChirho {
    TyVarChirho::plain_chirho(name_chirho(text_chirho))
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("TestChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn signature_chirho(ty_chirho: TypeChirho) -> DeclChirho {
    DeclChirho::TypeSigChirho {
        name_chirho: name_chirho("valueChirho"),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn check_chirho(
    module_chirho: &ModuleChirho,
    env_chirho: &NameEnvChirho,
) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    check_module_type_scope_chirho(module_chirho, env_chirho, &[], &mut diagnostics_chirho);
    diagnostics_chirho
}

#[test]
fn unknown_type_constructor_is_reported_once_per_source_use_chirho() {
    let missing_ty_chirho = TypeChirho::ConChirho(name_chirho("MissingChirho"));
    let module_chirho = module_chirho(vec![
        signature_chirho(missing_ty_chirho.clone()),
        signature_chirho(missing_ty_chirho),
    ]);
    let diagnostics_chirho = check_chirho(&module_chirho, &NameEnvChirho::new_chirho());
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type not in scope: `MissingChirho`"));
}

#[test]
fn local_imported_and_qualified_types_resolve_chirho() {
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "LocalChirho".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    env_chirho.bind_import_chirho(
        "ImportedChirho".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    env_chirho.bind_qualified_chirho(
        "LibChirho.QualifiedChirho".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let module_chirho = module_chirho(vec![
        signature_chirho(TypeChirho::ConChirho(name_chirho("LocalChirho"))),
        signature_chirho(TypeChirho::ConChirho(name_chirho("ImportedChirho"))),
        signature_chirho(TypeChirho::ConChirho(qualified_name_chirho(
            "LibChirho",
            "QualifiedChirho",
        ))),
    ]);
    assert!(check_chirho(&module_chirho, &env_chirho).is_empty_chirho());
}

#[test]
fn resolver_invokes_type_scope_after_collecting_definitions_chirho() {
    let module_chirho = module_chirho(vec![signature_chirho(TypeChirho::ConChirho(name_chirho(
        "MissingChirho",
    )))]);
    let result_chirho = resolve_module_chirho(&module_chirho);
    assert_eq!(result_chirho.diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{}", result_chirho.diagnostics_chirho).contains("MissingChirho"));
}

#[test]
fn resolver_binds_local_associated_family_in_type_namespace_chirho() {
    let family_use_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::ConChirho(name_chirho("FamilyChirho"))),
        arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = module_chirho(vec![
        DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: name_chirho("ClassChirho"),
            type_vars_chirho: vec![tyvar_chirho("aChirho")],
            methods_chirho: vec![],
            associated_tfs_chirho: vec![AssocTypeFamilyChirho {
                name_chirho: name_chirho("FamilyChirho"),
                type_vars_chirho: vec![name_chirho("aChirho")],
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        signature_chirho(family_use_chirho),
    ]);
    let result_chirho = resolve_module_chirho(&module_chirho);
    assert!(result_chirho.diagnostics_chirho.is_empty_chirho());
    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("FamilyChirho")
            .is_some()
    );
}

fn qualified_is_list_instance_chirho(import_spec_chirho: Option<ImportSpecChirho>) -> ModuleChirho {
    let mut module_chirho = module_chirho(vec![
        DeclChirho::DataDeclChirho {
            name_chirho: name_chirho("LocalListChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: qualified_name_chirho("GHC.Exts", "IsList"),
            types_chirho: vec![TypeChirho::ConChirho(name_chirho("LocalListChirho"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
                family_name_chirho: name_chirho("Item"),
                lhs_types_chirho: vec![TypeChirho::ConChirho(name_chirho("LocalListChirho"))],
                rhs_chirho: TypeChirho::ConChirho(name_chirho("LocalListChirho")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);
    module_chirho.imports_chirho = vec![ImportDeclChirho {
        module_chirho: name_chirho("GHC.Exts"),
        qualified_chirho: true,
        alias_chirho: None,
        spec_chirho: import_spec_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }];
    module_chirho
}

#[test]
fn qualified_class_import_scopes_visible_associated_type_inside_instance_chirho() {
    let module_chirho = qualified_is_list_instance_chirho(None);
    let result_chirho =
        resolve_module_with_imports_chirho(&module_chirho, &builtin_module_ifaces_chirho());
    assert!(result_chirho.diagnostics_chirho.is_empty_chirho());
    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("Item")
            .is_none()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho("GHC.Exts", "Item", NamespaceChirho::TypeChirho,)
            .is_some()
    );
}

#[test]
fn qualified_class_import_without_members_does_not_scope_associated_type_chirho() {
    let module_chirho = qualified_is_list_instance_chirho(Some(ImportSpecChirho {
        hiding_chirho: false,
        items_chirho: vec![ImportItemChirho::TyConChirho {
            name_chirho: name_chirho("IsList"),
            members_chirho: ExportMembersChirho::NoneChirho,
        }],
    }));
    let result_chirho =
        resolve_module_with_imports_chirho(&module_chirho, &builtin_module_ifaces_chirho());
    assert_eq!(result_chirho.diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{}", result_chirho.diagnostics_chirho).contains("Item"));
}

#[test]
fn associated_family_default_requires_a_declared_variable_chirho() {
    let module_chirho = module_chirho(vec![DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho("ClassChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![AssocTypeFamilyChirho {
            name_chirho: name_chirho("FamilyChirho"),
            type_vars_chirho: vec![name_chirho("aChirho")],
            default_rhs_chirho: Some(TypeChirho::VarChirho(name_chirho("bChirho"))),
            default_params_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    let diagnostics_chirho = check_chirho(&module_chirho, &NameEnvChirho::new_chirho());
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `bChirho`"));
}

#[test]
fn associated_family_default_uses_its_equation_binders_chirho() {
    let module_chirho = module_chirho(vec![DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho("ClassChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![AssocTypeFamilyChirho {
            name_chirho: name_chirho("FamilyChirho"),
            type_vars_chirho: vec![name_chirho("aChirho"), name_chirho("bChirho")],
            default_rhs_chirho: Some(TypeChirho::VarChirho(name_chirho("xChirho"))),
            default_params_chirho: vec![name_chirho("aChirho"), name_chirho("xChirho")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    assert!(check_chirho(&module_chirho, &NameEnvChirho::new_chirho()).is_empty_chirho());
}

#[test]
fn current_module_qualification_resolves_only_local_types_chirho() {
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "LocalChirho".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    env_chirho.bind_import_chirho(
        "ImportedChirho".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let module_chirho = module_chirho(vec![
        signature_chirho(TypeChirho::ConChirho(qualified_name_chirho(
            "TestChirho",
            "LocalChirho",
        ))),
        signature_chirho(TypeChirho::ConChirho(qualified_name_chirho(
            "TestChirho",
            "ImportedChirho",
        ))),
    ]);
    let diagnostics_chirho = check_chirho(&module_chirho, &env_chirho);
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("TestChirho.ImportedChirho"));
}

#[test]
fn datakinds_type_use_can_resolve_a_value_constructor_chirho() {
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "True".to_string(),
        NamespaceChirho::ValueChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let mut module_chirho = module_chirho(vec![signature_chirho(TypeChirho::ConChirho(
        name_chirho("True"),
    ))]);
    assert_eq!(
        check_chirho(&module_chirho, &env_chirho).error_count_chirho(),
        1
    );

    module_chirho
        .extensions_chirho
        .push("DataKinds".to_string());
    assert!(check_chirho(&module_chirho, &env_chirho).is_empty_chirho());
}

#[test]
fn datakinds_does_not_treat_a_constructor_as_a_class_chirho() {
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "True".to_string(),
        NamespaceChirho::ValueChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let mut module_chirho = module_chirho(vec![DeclChirho::ClassDeclChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho("True"),
            args_chirho: vec![TypeChirho::VarChirho(name_chirho("aChirho"))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        name_chirho: name_chirho("CChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    module_chirho
        .extensions_chirho
        .push("DataKinds".to_string());
    assert_eq!(
        check_chirho(&module_chirho, &env_chirho).error_count_chirho(),
        1
    );
}

#[test]
fn wired_in_type_constructors_need_no_import_chirho() {
    let module_chirho = module_chirho(
        [
            "~", "*", ":", "':", "[]", "()", "->", "(->)", "(,)", "(,,)", "(##)", "(# #)", "(#,#)",
            "(#,,#)", "(#|#)",
        ]
        .into_iter()
        .map(|name_text_chirho| {
            signature_chirho(TypeChirho::ConChirho(name_chirho(name_text_chirho)))
        })
        .collect(),
    );
    assert!(check_chirho(&module_chirho, &NameEnvChirho::new_chirho()).is_empty_chirho());
}

#[test]
fn no_star_is_type_restores_star_to_namespace_lookup_chirho() {
    let mut module_chirho = module_chirho(vec![signature_chirho(TypeChirho::ConChirho(
        name_chirho("*"),
    ))]);
    module_chirho
        .extensions_chirho
        .push("NoStarIsType".to_string());
    let mut env_chirho = NameEnvChirho::new_chirho();
    assert_eq!(
        check_chirho(&module_chirho, &env_chirho).error_count_chirho(),
        1
    );

    env_chirho.bind_chirho(
        "*".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(check_chirho(&module_chirho, &env_chirho).is_empty_chirho());
}

#[test]
fn datakinds_does_not_promote_a_value_function_operator_chirho() {
    let mut module_chirho = module_chirho(vec![signature_chirho(TypeChirho::ConChirho(
        name_chirho("*"),
    ))]);
    module_chirho
        .extensions_chirho
        .extend(["DataKinds".to_string(), "NoStarIsType".to_string()]);
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "*".to_string(),
        NamespaceChirho::ValueChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        check_chirho(&module_chirho, &env_chirho).error_count_chirho(),
        1
    );
}

#[test]
fn wired_in_promoted_constructors_need_no_import_chirho() {
    let module_chirho = module_chirho(
        [":", "':", "[]", "()", "(,)", "(,,)"]
            .into_iter()
            .map(|name_text_chirho| {
                signature_chirho(TypeChirho::PromotedConChirho {
                    name_chirho: name_chirho(name_text_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                })
            })
            .collect(),
    );
    assert!(check_chirho(&module_chirho, &NameEnvChirho::new_chirho()).is_empty_chirho());
}

#[test]
fn synonym_rhs_requires_lexically_bound_type_variables_chirho() {
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        rhs_chirho: TypeChirho::TupleChirho {
            elements_chirho: vec![
                TypeChirho::VarChirho(name_chirho("aChirho")),
                TypeChirho::VarChirho(name_chirho("bChirho")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `bChirho`"));
}

#[test]
fn family_result_kind_resolves_type_namespace_names_chirho() {
    let decl_chirho = DeclChirho::TypeFamilyDeclChirho {
        name_chirho: name_chirho("FamilyChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        result_kind_chirho: Some(TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(name_chirho("Constraint"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        equations_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("Constraint"));
}

#[test]
fn deriving_via_type_uses_are_resolved_chirho() {
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "ClassChirho".to_string(),
        NamespaceChirho::TypeChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let mut module_chirho = module_chirho(vec![]);
    module_chirho.deriving_via_chirho.push((
        name_chirho("TargetChirho"),
        name_chirho("ClassChirho"),
        TypeChirho::ConChirho(name_chirho("MissingViaChirho")),
    ));
    let diagnostics_chirho = check_chirho(&module_chirho, &env_chirho);
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("MissingViaChirho"));
}

#[test]
fn explicit_forall_binds_its_body_variable_chirho() {
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::ForallChirho {
            vars_chirho: vec![tyvar_chirho("aChirho")],
            body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn outermost_signature_forall_requires_every_variable_binder_chirho() {
    let ty_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![tyvar_chirho("aChirho")],
        body_chirho: Box::new(TypeChirho::TupleChirho {
            elements_chirho: vec![
                TypeChirho::VarChirho(name_chirho("aChirho")),
                TypeChirho::VarChirho(name_chirho("bChirho")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![signature_chirho(ty_chirho)]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `bChirho`"));
}

#[test]
fn outermost_data_kind_forall_requires_every_variable_binder_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: Some(
            haskelujah_ast_chirho::decl_chirho::DataKindSigChirho::ResultChirho(
                TypeChirho::ForallChirho {
                    vars_chirho: vec![tyvar_chirho("aChirho")],
                    body_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::VarChirho(name_chirho("bChirho"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
        ),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `bChirho`"));
}

#[test]
fn signature_without_outermost_forall_implicitly_quantifies_chirho() {
    let ty_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
        mult_chirho: None,
        result_chirho: Box::new(TypeChirho::VarChirho(name_chirho("bChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![signature_chirho(ty_chirho)]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn both_declaration_kind_annotations_are_checked_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: Some(
            haskelujah_ast_chirho::decl_chirho::DataKindSigChirho::StandaloneChirho {
                signature_chirho: TypeChirho::ConChirho(name_chirho("MissingCompleteChirho")),
                result_chirho: Some(TypeChirho::ConChirho(name_chirho("MissingTailChirho"))),
            },
        ),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 2);
    let message_chirho = diagnostics_chirho.to_string();
    assert!(
        message_chirho.contains("MissingCompleteChirho")
            && message_chirho.contains("MissingTailChirho"),
        "{message_chirho}"
    );
}

#[test]
fn standalone_kind_forall_cannot_capture_a_declaration_head_binder_chirho() {
    let signature_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![tyvar_chirho("aChirho")],
        body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("kChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![tyvar_chirho("kChirho")],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: Some(
            haskelujah_ast_chirho::decl_chirho::DataKindSigChirho::StandaloneChirho {
                signature_chirho: signature_chirho.clone(),
                result_chirho: Some(signature_chirho),
            },
        ),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut inline_only_chirho = decl_chirho.clone();
    if let DeclChirho::DataDeclChirho {
        kind_sig_chirho, ..
    } = &mut inline_only_chirho
    {
        let result_chirho = kind_sig_chirho
            .as_ref()
            .unwrap()
            .result_chirho()
            .unwrap()
            .clone();
        *kind_sig_chirho = Some(
            haskelujah_ast_chirho::decl_chirho::DataKindSigChirho::ResultChirho(result_chirho),
        );
    }
    assert!(
        check_chirho(
            &module_chirho(vec![inline_only_chirho]),
            &NameEnvChirho::new_chirho()
        )
        .is_empty_chirho(),
        "the inline annotation really can see the head binder"
    );
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    // The complete signature's explicit forall does not bind k. The inline
    // signature can see the declaration's k, but cannot excuse the former.
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(
        diagnostics_chirho
            .to_string()
            .contains("type variable not in scope: `kChirho`")
    );
}

#[test]
fn parenthesized_forall_keeps_outer_free_variables_implicit_chirho() {
    let ty_chirho = TypeChirho::ParenChirho {
        inner_chirho: Box::new(TypeChirho::ForallChirho {
            vars_chirho: vec![tyvar_chirho("aChirho")],
            body_chirho: Box::new(TypeChirho::TupleChirho {
                elements_chirho: vec![
                    TypeChirho::VarChirho(name_chirho("aChirho")),
                    TypeChirho::VarChirho(name_chirho("bChirho")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![signature_chirho(ty_chirho)]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn required_forall_keeps_outer_free_variables_implicit_chirho() {
    let ty_chirho = TypeChirho::RequiredForallChirho {
        vars_chirho: vec![tyvar_chirho("aChirho")],
        body_chirho: Box::new(TypeChirho::TupleChirho {
            elements_chirho: vec![
                TypeChirho::VarChirho(name_chirho("aChirho")),
                TypeChirho::VarChirho(name_chirho("bChirho")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![signature_chirho(ty_chirho)]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn nested_forall_shadowing_restores_the_outer_binder_chirho() {
    let inner_forall_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![tyvar_chirho("aChirho")],
        body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::ForallChirho {
            vars_chirho: vec![tyvar_chirho("aChirho")],
            body_chirho: Box::new(TypeChirho::TupleChirho {
                elements_chirho: vec![
                    inner_forall_chirho,
                    TypeChirho::VarChirho(name_chirho("aChirho")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn promoted_constructor_uses_the_value_namespace_chirho() {
    let mut env_chirho = NameEnvChirho::new_chirho();
    env_chirho.bind_chirho(
        "PresentChirho".to_string(),
        NamespaceChirho::ValueChirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let module_chirho = module_chirho(vec![
        signature_chirho(TypeChirho::PromotedConChirho {
            name_chirho: name_chirho("PresentChirho"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        signature_chirho(TypeChirho::PromotedConChirho {
            name_chirho: name_chirho("AbsentChirho"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
    ]);
    let diagnostics_chirho = check_chirho(&module_chirho, &env_chirho);
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("AbsentChirho"));
}

#[test]
fn missing_promoted_constructor_defers_when_import_inventory_is_incomplete_chirho() {
    let mut module_chirho = module_chirho(vec![signature_chirho(TypeChirho::PromotedConChirho {
        name_chirho: name_chirho("ImportedFamilyConstructorChirho"),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    })]);
    module_chirho.imports_chirho.push(ImportDeclChirho {
        module_chirho: name_chirho("SourceModuleChirho"),
        qualified_chirho: false,
        alias_chirho: None,
        spec_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });

    assert!(check_chirho(&module_chirho, &NameEnvChirho::new_chirho()).is_empty_chirho());
}

#[test]
fn type_data_alias_recovery_does_not_invent_a_type_use_chirho() {
    let mut module_chirho = module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("LetterChirho"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::ConChirho(name_chirho("AChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    module_chirho.extensions_chirho.push("TypeData".to_string());

    assert!(check_chirho(&module_chirho, &NameEnvChirho::new_chirho()).is_empty_chirho());
}

#[test]
fn ordinary_record_field_type_is_still_resolved_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::RecordChirho {
            name_chirho: name_chirho("MkTChirho"),
            fields_chirho: vec![FieldDeclChirho {
                names_chirho: vec![name_chirho("fieldChirho")],
                ty_chirho: TypeChirho::ConChirho(name_chirho("MissingChirho")),
                strictness_chirho: StrictnessChirho::LazyChirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .error_count_chirho(),
        1
    );
}

#[test]
fn parser_sensitive_rank_n_record_field_is_deferred_chirho() {
    let malformed_field_ty_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(TypeChirho::ForallChirho {
            vars_chirho: vec![tyvar_chirho("aChirho")],
            body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        mult_chirho: None,
        result_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let decl_chirho = DeclChirho::NewtypeDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![],
        constructor_chirho: ConDeclChirho::RecordChirho {
            name_chirho: name_chirho("MkTChirho"),
            fields_chirho: vec![FieldDeclChirho {
                names_chirho: vec![name_chirho("fieldChirho")],
                ty_chirho: malformed_field_ty_chirho,
                strictness_chirho: StrictnessChirho::LazyChirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn required_forall_record_field_remains_checkable_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::RecordChirho {
            name_chirho: name_chirho("MkTChirho"),
            fields_chirho: vec![FieldDeclChirho {
                names_chirho: vec![name_chirho("fieldChirho")],
                ty_chirho: TypeChirho::RequiredForallChirho {
                    vars_chirho: vec![tyvar_chirho("typeChirho")],
                    body_chirho: Box::new(TypeChirho::ConChirho(name_chirho("MissingChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                strictness_chirho: StrictnessChirho::LazyChirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("MissingChirho"));
}

#[test]
fn explicit_forall_kind_annotation_needs_an_earlier_binder_chirho() {
    let annotated_chirho = TyVarChirho::annotated_chirho(
        name_chirho("aChirho"),
        AstKindChirho::VarChirho("kChirho".to_string()),
    );
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::ForallChirho {
            vars_chirho: vec![annotated_chirho],
            body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `kChirho`"));
}

#[test]
fn explicit_forall_kind_annotation_sees_previous_binders_chirho() {
    let annotated_chirho = TyVarChirho::annotated_chirho(
        name_chirho("aChirho"),
        AstKindChirho::VarChirho("kChirho".to_string()),
    );
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::ForallChirho {
            vars_chirho: vec![tyvar_chirho("kChirho"), annotated_chirho],
            body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn declaration_head_kind_variables_are_implicitly_bound_chirho() {
    let kinded_param_chirho = TyVarChirho::annotated_chirho(
        name_chirho("aChirho"),
        AstKindChirho::VarChirho("kChirho".to_string()),
    );
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![kinded_param_chirho],
        rhs_chirho: TypeChirho::VarChirho(name_chirho("kChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn declaration_head_underscore_kind_variable_is_implicitly_bound_chirho() {
    let kinded_param_chirho = TyVarChirho::annotated_chirho(
        name_chirho("aChirho"),
        AstKindChirho::VarChirho("_kindChirho".to_string()),
    );
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho("AliasChirho"),
        type_vars_chirho: vec![kinded_param_chirho],
        rhs_chirho: TypeChirho::VarChirho(name_chirho("_kindChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn ordinary_constructor_free_variable_requires_existential_extension_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: name_chirho("MkTChirho"),
            fields_chirho: vec![(
                StrictnessChirho::LazyChirho,
                TypeChirho::VarChirho(name_chirho("bChirho")),
            )],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut without_chirho = module_chirho(vec![decl_chirho.clone()]);
    let env_chirho = NameEnvChirho::new_chirho();
    assert_eq!(
        check_chirho(&without_chirho, &env_chirho).error_count_chirho(),
        1
    );

    without_chirho
        .extensions_chirho
        .push("ExistentialQuantification".to_string());
    assert!(check_chirho(&without_chirho, &env_chirho).is_empty_chirho());
}

#[test]
fn dropped_existential_prefix_defers_only_its_constructor_fields_chirho() {
    let declaration_span_chirho = SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(10),
        ByteOffsetChirho::new_chirho(50),
    );
    let constructor_name_span_chirho = SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(25),
        ByteOffsetChirho::new_chirho(33),
    );
    let existential_name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        "MkExistChirho",
        constructor_name_span_chirho,
    ));
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: existential_name_chirho,
            fields_chirho: vec![(
                StrictnessChirho::LazyChirho,
                TypeChirho::VarChirho(name_chirho("hiddenChirho")),
            )],
            span_chirho: declaration_span_chirho,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: declaration_span_chirho,
    };

    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn gadt_outer_forall_requires_every_variable_binder_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        constructors_chirho: vec![ConDeclChirho::GadtChirho {
            name_chirho: name_chirho("MkTChirho"),
            ty_chirho: TypeChirho::ForallChirho {
                vars_chirho: vec![tyvar_chirho("bChirho")],
                body_chirho: Box::new(TypeChirho::TupleChirho {
                    elements_chirho: vec![
                        TypeChirho::VarChirho(name_chirho("aChirho")),
                        TypeChirho::VarChirho(name_chirho("bChirho")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `aChirho`"));
}

#[test]
fn gadt_without_outer_forall_quantifies_independently_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        constructors_chirho: vec![ConDeclChirho::GadtChirho {
            name_chirho: name_chirho("MkTChirho"),
            ty_chirho: TypeChirho::VarChirho(name_chirho("aChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_chirho(
            &module_chirho(vec![decl_chirho]),
            &NameEnvChirho::new_chirho(),
        )
        .is_empty_chirho()
    );
}

#[test]
fn unknown_superclass_is_resolved_in_the_type_namespace_chirho() {
    let decl_chirho = DeclChirho::ClassDeclChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho("MissingClassChirho"),
            args_chirho: vec![TypeChirho::VarChirho(name_chirho("aChirho"))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        name_chirho: name_chirho("CChirho"),
        type_vars_chirho: vec![tyvar_chirho("aChirho")],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_chirho(
        &module_chirho(vec![decl_chirho]),
        &NameEnvChirho::new_chirho(),
    );
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("MissingClassChirho"));
}
