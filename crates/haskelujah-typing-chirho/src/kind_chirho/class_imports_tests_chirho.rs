// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin class kinds through the imports wrapper (K01 C). Each module mirrors a program
//! GHC 9.14.1 measured in `oracle-classes-chirho` (CH holds, CN negatives) or a corpus
//! representative (T1633, T11464, UnliftedNewtypesFail); the scope stands in for naming.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{ClassMethodChirho, ConDeclChirho, StrictnessChirho};
use haskelujah_ast_chirho::name_chirho::{BuiltinClassChirho, NameChirho, RawNameChirho};

const HEAD_ERROR_CHIRHO: &str = "kind mismatch in the instance head";
const TYPE_POSITION_ERROR_CHIRHO: &str = "is a constraint";

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

fn con_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(name_chirho(text_chirho))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

fn app_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn fun_chirho(arg_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::FunChirho {
        arg_chirho: Box::new(arg_chirho),
        mult_chirho: None,
        result_chirho: Box::new(result_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `data <name> <binders> = Mk<name> <fields>`.
fn data_chirho(
    type_text_chirho: &str,
    binders_chirho: &[&str],
    fields_chirho: Vec<TypeChirho>,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_text_chirho),
        type_vars_chirho: binders_chirho
            .iter()
            .map(|binder_chirho| TyVarChirho::plain_chirho(name_chirho(binder_chirho)))
            .collect(),
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: name_chirho(&format!("Mk{type_text_chirho}")),
            fields_chirho: fields_chirho
                .into_iter()
                .map(|field_chirho| (StrictnessChirho::LazyChirho, field_chirho))
                .collect(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `instance <class> <head>` with no methods.
fn instance_chirho(class_chirho: NameChirho, head_chirho: TypeChirho) -> DeclChirho {
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho,
        types_chirho: vec![head_chirho],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn signature_chirho(binding_text_chirho: &str, ty_chirho: TypeChirho) -> DeclChirho {
    DeclChirho::TypeSigChirho {
        name_chirho: name_chirho(binding_text_chirho),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn builtin_class_chirho(name_text_chirho: &'static str, arity_chirho: u8) -> BuiltinClassChirho {
    BuiltinClassChirho {
        module_chirho: "GHC.Base",
        name_chirho: name_text_chirho,
        parameter_arities_chirho: if arity_chirho == 0 { &[0] } else { &[1] },
    }
}

/// A scope binding each spelling to a builtin class of the given parameter arity.
fn scope_chirho(spellings_chirho: &[(&str, &'static str, u8)]) -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(
        spellings_chirho
            .iter()
            .map(|(spelling_chirho, class_text_chirho, arity_chirho)| {
                (
                    spelling_chirho.to_string(),
                    builtin_class_chirho(*class_text_chirho, *arity_chirho),
                )
            })
            .collect(),
    )
}

fn module_chirho(extensions_chirho: &[&str], decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("ClassImportsChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: extensions_chirho
            .iter()
            .map(|extension_chirho| extension_chirho.to_string())
            .collect(),
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

/// Error count and rendered diagnostics of one module's kind pass under `scope_chirho`.
fn kind_errors_chirho(
    scope_chirho: &BuiltinClassScopeChirho,
    module_chirho: &ModuleChirho,
) -> (usize, String) {
    let result_chirho = infer_module_kinds_with_imports_chirho(
        module_chirho,
        KindImportsChirho {
            builtin_classes_chirho: scope_chirho,
        },
    );
    (
        result_chirho.diagnostics_chirho.error_count_chirho(),
        result_chirho.diagnostics_chirho.to_string(),
    )
}

fn flag_chirho() -> DeclChirho {
    data_chirho("FlagChirho", &[], vec![])
}

fn either_twin_chirho() -> DeclChirho {
    data_chirho(
        "EitherTwinChirho",
        &["e", "a"],
        vec![var_chirho("e"), var_chirho("a")],
    )
}

fn box_chirho() -> DeclChirho {
    data_chirho("BoxChirho", &["a"], vec![var_chirho("a")])
}

#[test]
fn a_builtin_class_head_of_another_kind_is_reported_under_any_extensions_chirho() {
    let functor_chirho = scope_chirho(&[("Functor", "Functor", 1)]);
    let eq_chirho = scope_chirho(&[("Eq", "Eq", 0)]);
    let foldable_chirho = scope_chirho(&[("Foldable", "Foldable", 1)]);
    for (label_chirho, scope_chirho, module_chirho) in [
        (
            "CN01",
            &functor_chirho,
            module_chirho(
                &["FlexibleInstances"],
                vec![
                    flag_chirho(),
                    instance_chirho(name_chirho("Functor"), con_chirho("FlagChirho")),
                ],
            ),
        ),
        (
            "CN02",
            &eq_chirho,
            module_chirho(
                &["PolyKinds"],
                vec![
                    either_twin_chirho(),
                    instance_chirho(
                        name_chirho("Eq"),
                        app_chirho(con_chirho("EitherTwinChirho"), var_chirho("e")),
                    ),
                ],
            ),
        ),
        (
            "CN04",
            &foldable_chirho,
            module_chirho(
                &[],
                vec![
                    data_chirho("CountChirho", &[], vec![con_chirho("Int")]),
                    instance_chirho(name_chirho("Foldable"), con_chirho("CountChirho")),
                ],
            ),
        ),
        (
            "T1633",
            &functor_chirho,
            module_chirho(
                &["FlexibleInstances"],
                vec![instance_chirho(name_chirho("Functor"), con_chirho("Bool"))],
            ),
        ),
        (
            "T11464",
            &eq_chirho,
            module_chirho(
                &["MagicHash", "PolyKinds"],
                vec![instance_chirho(
                    name_chirho("Eq"),
                    app_chirho(con_chirho("Either"), var_chirho("a")),
                )],
            ),
        ),
    ] {
        let (count_chirho, text_chirho) = kind_errors_chirho(scope_chirho, &module_chirho);
        assert_eq!(count_chirho, 1, "{label_chirho}: {text_chirho}");
        assert!(
            text_chirho.contains(HEAD_ERROR_CHIRHO),
            "{label_chirho}: {text_chirho}"
        );
    }
}

#[test]
fn a_qualified_head_is_judged_only_by_its_proven_spelling_chirho() {
    let module_chirho = module_chirho(
        &[],
        vec![
            flag_chirho(),
            instance_chirho(
                qualified_name_chirho("P", "Functor"),
                con_chirho("FlagChirho"),
            ),
        ],
    );
    let (count_chirho, text_chirho) = kind_errors_chirho(
        &scope_chirho(&[("P.Functor", "Functor", 1)]),
        &module_chirho,
    );
    assert_eq!(count_chirho, 1, "CN03: {text_chirho}");
    let (count_chirho, text_chirho) =
        kind_errors_chirho(&scope_chirho(&[("Functor", "Functor", 1)]), &module_chirho);
    assert_eq!(
        count_chirho, 0,
        "an unproven qualified spelling: {text_chirho}"
    );
}

#[test]
fn a_builtin_class_written_as_a_field_is_a_constraint_chirho() {
    for (label_chirho, spelling_chirho, class_chirho) in [
        ("UnliftedNewtypesFail", "Show", name_chirho("Show")),
        ("CN05", "P.Show", qualified_name_chirho("P", "Show")),
    ] {
        let module_chirho = module_chirho(
            &[],
            vec![data_chirho(
                "BazChirho",
                &[],
                vec![app_chirho(
                    TypeChirho::ConChirho(class_chirho),
                    con_chirho("Int"),
                )],
            )],
        );
        let (count_chirho, text_chirho) = kind_errors_chirho(
            &scope_chirho(&[(spelling_chirho, "Show", 0)]),
            &module_chirho,
        );
        assert_eq!(count_chirho, 1, "{label_chirho}: {text_chirho}");
        assert!(
            text_chirho.contains(TYPE_POSITION_ERROR_CHIRHO),
            "{label_chirho}: {text_chirho}"
        );
    }
}

#[test]
fn proven_builtin_heads_of_the_right_kind_and_unjudged_shapes_hold_chirho() {
    let scope_chirho = scope_chirho(&[
        ("Functor", "Functor", 1),
        ("P.Functor", "Functor", 1),
        ("Show", "Show", 0),
    ]);
    let module_chirho = module_chirho(
        &["FlexibleInstances"],
        vec![
            either_twin_chirho(),
            box_chirho(),
            // CH01: a resolved `* -> *` argument under FlexibleInstances.
            instance_chirho(
                name_chirho("Functor"),
                app_chirho(con_chirho("EitherTwinChirho"), var_chirho("e")),
            ),
            // CH04: a qualified class at an own type.
            instance_chirho(
                qualified_name_chirho("P", "Functor"),
                con_chirho("BoxChirho"),
            ),
            // CH06: a variable head stays unjudged.
            instance_chirho(
                name_chirho("Show"),
                app_chirho(var_chirho("f"), var_chirho("a")),
            ),
            // An imported head: its kind would be learned from a use, so it stays unjudged.
            instance_chirho(
                name_chirho("Functor"),
                app_chirho(con_chirho("ImportedChirho"), var_chirho("a")),
            ),
            // List, tuple and function heads are `*`.
            instance_chirho(
                name_chirho("Show"),
                TypeChirho::ListChirho {
                    element_chirho: Box::new(var_chirho("a")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
            instance_chirho(
                name_chirho("Show"),
                fun_chirho(var_chirho("a"), var_chirho("b")),
            ),
        ],
    );
    let (count_chirho, text_chirho) = kind_errors_chirho(&scope_chirho, &module_chirho);
    assert_eq!(count_chirho, 0, "{text_chirho}");
}

#[test]
fn a_local_homonym_is_never_seeded_as_the_builtin_class_chirho() {
    // CH05: a local `class Functor a` of kind `* -> Constraint`. Naming excludes its
    // spelling; even a scope that wrongly kept it must leave the local class in charge.
    let module_chirho = module_chirho(
        &[],
        vec![
            DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: name_chirho("Functor"),
                type_vars_chirho: vec![name_chirho("a").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: name_chirho("describeChirho"),
                    ty_chirho: fun_chirho(var_chirho("a"), con_chirho("String")),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            instance_chirho(name_chirho("Functor"), con_chirho("Bool")),
        ],
    );
    let (count_chirho, text_chirho) =
        kind_errors_chirho(&scope_chirho(&[("Functor", "Functor", 1)]), &module_chirho);
    assert_eq!(count_chirho, 0, "{text_chirho}");
}

#[test]
fn builtin_class_kinds_hold_where_a_constraint_is_expected_chirho() {
    // P01 and P02 with Show and Eq now known: `DictChirho (Show a)` and `Proxy (Eq Int)`.
    let module_chirho = module_chirho(
        &[],
        vec![
            DeclChirho::DataDeclChirho {
                name_chirho: name_chirho("DictChirho"),
                type_vars_chirho: vec![TyVarChirho::annotated_chirho(
                    name_chirho("c"),
                    AstKindChirho::ConstraintChirho,
                )],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            signature_chirho(
                "showWithChirho",
                fun_chirho(
                    app_chirho(
                        con_chirho("DictChirho"),
                        app_chirho(con_chirho("Show"), var_chirho("a")),
                    ),
                    fun_chirho(var_chirho("a"), con_chirho("String")),
                ),
            ),
            signature_chirho(
                "proxyChirho",
                app_chirho(
                    con_chirho("Proxy"),
                    app_chirho(con_chirho("Eq"), con_chirho("Int")),
                ),
            ),
        ],
    );
    let (count_chirho, text_chirho) = kind_errors_chirho(
        &scope_chirho(&[("Show", "Show", 0), ("Eq", "Eq", 0)]),
        &module_chirho,
    );
    assert_eq!(count_chirho, 0, "{text_chirho}");
}

#[test]
fn the_direct_api_knows_no_builtin_class_chirho() {
    let module_chirho = module_chirho(
        &["FlexibleInstances"],
        vec![
            flag_chirho(),
            instance_chirho(name_chirho("Functor"), con_chirho("FlagChirho")),
        ],
    );
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "{}",
        result_chirho.diagnostics_chirho
    );
}
