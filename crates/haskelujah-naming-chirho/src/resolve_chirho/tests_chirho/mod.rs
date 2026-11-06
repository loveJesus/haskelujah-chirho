// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{ImportDeclChirho, ImportSpecChirho};
use haskelujah_ast_chirho::name_chirho::RawNameChirho;

use crate::iface_chirho::{IfaceTypeChirho, IfaceValueChirho};

mod imports_chirho;

fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn mk_module_chirho(
    decls_chirho: Vec<DeclChirho>,
    imports_chirho: Vec<ImportDeclChirho>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: dummy_name_chirho("Main"),
        exports_chirho: None,
        imports_chirho,
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn prelude_import_chirho() -> ImportDeclChirho {
    ImportDeclChirho {
        package_chirho: None,
        module_chirho: dummy_name_chirho("Prelude"),
        source_chirho: false,
        qualified_chirho: false,
        alias_chirho: None,
        spec_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

// ---------------------------------------------------------------
// Single-module tests (no imports)
// ---------------------------------------------------------------

#[test]
fn resolve_simple_module_chirho() {
    let module_chirho = mk_module_chirho(
        vec![
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("Red"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                        prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(
                        ),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("Green"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                        prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(
                        ),
                    },
                ],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("main"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        vec![],
    );

    let result_chirho = resolve_module_chirho(&module_chirho);

    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("Color")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("Red")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("Green")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("main")
            .is_some()
    );
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn resolve_class_methods_chirho() {
    let module_chirho = mk_module_chirho(
        vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("Describable"),
            type_vars_chirho: vec![dummy_name_chirho("a").into()],
            methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
                name_chirho: dummy_name_chirho("describe"),
                ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::VarChirho(
                    dummy_name_chirho("a"),
                ),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    );

    let result_chirho = resolve_module_chirho(&module_chirho);

    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("Describable")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("describe")
            .is_some()
    );
}

// ---------------------------------------------------------------
// Import resolution tests
// ---------------------------------------------------------------

/// Test: record field names are registered as value bindings.
#[test]
fn resolve_record_field_accessors_chirho() {
    use haskelujah_ast_chirho::decl_chirho::FieldDeclChirho;

    let module_chirho = mk_module_chirho(
        vec![DeclChirho::DataDeclChirho {
            name_chirho: dummy_name_chirho("Person"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::RecordChirho {
                name_chirho: dummy_name_chirho("MkPerson"),
                fields_chirho: vec![
                    FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![dummy_name_chirho("personName")],
                        ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                            dummy_name_chirho("String"),
                        ),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![dummy_name_chirho("personAge")],
                        ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                            dummy_name_chirho("Int"),
                        ),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![prelude_import_chirho()],
    );

    let result_chirho = resolve_module_with_imports_chirho(
        &module_chirho,
        &crate::iface_chirho::builtin_module_ifaces_chirho(),
    );
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());

    // Type should be registered
    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("Person")
            .is_some()
    );
    // Constructor should be registered
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("MkPerson")
            .is_some()
    );
    // Field accessors should be registered as values
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("personName")
            .is_some(),
        "personName field accessor should be in scope"
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("personAge")
            .is_some(),
        "personAge field accessor should be in scope"
    );
}

/// Test: top-level pattern binding binds variables.
#[test]
fn resolve_top_level_pat_bind_chirho() {
    use haskelujah_ast_chirho::expr_chirho::RhsChirho;

    let module_chirho = mk_module_chirho(
        vec![DeclChirho::PatBindChirho {
            pat_chirho: PatChirho::TupleChirho {
                elements_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("x")),
                    PatChirho::VarChirho(dummy_name_chirho("y")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(
                haskelujah_ast_chirho::expr_chirho::ExprChirho::TupleChirho {
                    elements_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    );

    let result_chirho = resolve_module_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        result_chirho.env_chirho.lookup_value_chirho("x").is_some(),
        "x from (x, y) = ... should be in scope"
    );
    assert!(
        result_chirho.env_chirho.lookup_value_chirho("y").is_some(),
        "y from (x, y) = ... should be in scope"
    );
}

/// Test: as-pattern in top-level pat bind.
#[test]
fn resolve_as_pattern_bind_chirho() {
    use haskelujah_ast_chirho::expr_chirho::RhsChirho;

    let module_chirho = mk_module_chirho(
        vec![DeclChirho::PatBindChirho {
            pat_chirho: PatChirho::AsChirho {
                name_chirho: dummy_name_chirho("whole"),
                pattern_chirho: Box::new(PatChirho::ConChirho {
                    con_chirho: dummy_name_chirho("Just"),
                    type_args_chirho: vec![],
                    args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("inner"))],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(
                haskelujah_ast_chirho::expr_chirho::ExprChirho::TupleChirho {
                    elements_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    );

    let result_chirho = resolve_module_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("whole")
            .is_some(),
        "whole from whole@(Just inner) should be in scope"
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("inner")
            .is_some(),
        "inner from whole@(Just inner) should be in scope"
    );
}

/// Test: newtype record field accessor is registered.
#[test]
fn resolve_newtype_record_field_chirho() {
    use haskelujah_ast_chirho::decl_chirho::FieldDeclChirho;

    let module_chirho = mk_module_chirho(
        vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: dummy_name_chirho("Age"),
            type_vars_chirho: vec![],
            constructor_chirho: ConDeclChirho::RecordChirho {
                name_chirho: dummy_name_chirho("MkAge"),
                fields_chirho: vec![FieldDeclChirho {
                    multiplicity_chirho: None,
                    names_chirho: vec![dummy_name_chirho("getAge")],
                    ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                        dummy_name_chirho("Int"),
                    ),
                    strictness_chirho:
                        haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            },
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![prelude_import_chirho()],
    );

    let result_chirho = resolve_module_with_imports_chirho(
        &module_chirho,
        &crate::iface_chirho::builtin_module_ifaces_chirho(),
    );
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(result_chirho.env_chirho.lookup_type_chirho("Age").is_some());
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("MkAge")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_value_chirho("getAge")
            .is_some(),
        "getAge field accessor for newtype record should be in scope"
    );
}

// ---------------------------------------------------------------
// Orphan instance detection tests
// ---------------------------------------------------------------

/// Test: instance with locally-defined class is NOT orphan.
#[test]
fn orphan_local_class_not_orphan_chirho() {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

    let module_chirho = mk_module_chirho(
        vec![
            DeclChirho::ClassDeclChirho {
                standalone_kind_chirho: None,
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("MyClass"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                methods_chirho: vec![],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                kind_args_chirho: Vec::new(),
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("MyClass"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        vec![],
    );

    let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
    assert!(
        warnings_chirho.is_empty_chirho(),
        "instance for locally-defined class should not be orphan"
    );
}

/// Test: instance with locally-defined type is NOT orphan.
#[test]
fn orphan_local_type_not_orphan_chirho() {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

    let module_chirho = mk_module_chirho(
        vec![
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                kind_args_chirho: Vec::new(),
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Show"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Color"))],
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        vec![],
    );

    let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
    assert!(
        warnings_chirho.is_empty_chirho(),
        "instance for locally-defined type should not be orphan"
    );
}

/// Test: instance where neither class nor type is local IS orphan.
#[test]
fn orphan_foreign_class_and_type_chirho() {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

    let module_chirho = mk_module_chirho(
        vec![DeclChirho::InstanceDeclChirho {
            kind_args_chirho: Vec::new(),
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("Show"),
            types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    );

    let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
    assert_eq!(
        warnings_chirho.len_chirho(),
        1,
        "instance Show Int without local class/type should be orphan"
    );
    assert_eq!(warnings_chirho.warning_count_chirho(), 1);
    let msg_chirho = warnings_chirho.diagnostics_chirho()[0]
        .message_chirho
        .clone();
    assert!(
        msg_chirho.contains("orphan instance"),
        "message should mention 'orphan instance'"
    );
}

/// Test: instance with local newtype in App head is NOT orphan.
#[test]
fn orphan_local_newtype_in_app_not_orphan_chirho() {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

    let module_chirho = mk_module_chirho(
        vec![
            DeclChirho::NewtypeDeclChirho {
                name_chirho: dummy_name_chirho("Wrapper"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: dummy_name_chirho("MkWrapper"),
                    fields_chirho: vec![(
                        haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(dummy_name_chirho("a")),
                    )],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                    prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                kind_args_chirho: Vec::new(),
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Show"),
                types_chirho: vec![TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Wrapper"))),
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        vec![],
    );

    let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
    assert!(
        warnings_chirho.is_empty_chirho(),
        "instance Show (Wrapper a) with local Wrapper should not be orphan"
    );
}

/// Test: multiple orphan instances produce multiple warnings.
#[test]
fn orphan_multiple_warnings_chirho() {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

    let module_chirho = mk_module_chirho(
        vec![
            DeclChirho::InstanceDeclChirho {
                kind_args_chirho: Vec::new(),
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Show"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Int"))],
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                kind_args_chirho: Vec::new(),
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Eq"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Bool"))],
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        vec![],
    );

    let warnings_chirho = check_orphan_instances_chirho(&module_chirho);
    assert_eq!(warnings_chirho.len_chirho(), 2);
}
