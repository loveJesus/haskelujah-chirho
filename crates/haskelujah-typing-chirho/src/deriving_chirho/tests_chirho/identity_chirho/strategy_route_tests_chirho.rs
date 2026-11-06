// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving strategy routes (row 694): an explicit `newtype` takes the representation's
//! instance and never a stock one, `stock` and `anyclass` stay themselves, an unwritten
//! strategy keeps the legacy policy, and a GND head drops the variables a builtin class's
//! parameter kind needs (one for `Functor`, two for `MonadTrans`). A request K01 proves names
//! the builtin `Typeable` takes no route at all (#32106).

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    DerivedClassChirho, DerivingStrategyChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::name_chirho::{BuiltinClassChirho, CompilerClassChirho};
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

use super::*;

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: var_name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    }
}

fn con_ty_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(var_name_chirho(text_chirho))
}

fn var_ty_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(var_name_chirho(text_chirho))
}

fn app_ty_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: gen_span_chirho(),
    }
}

fn derived_chirho(
    class_chirho: &str,
    strategy_chirho: Option<DerivingStrategyChirho>,
) -> DerivedClassChirho {
    DerivedClassChirho::new_chirho(var_name_chirho(class_chirho), strategy_chirho)
}

/// `class_chirho` applied to `Int`.
fn applied_chirho(
    class_chirho: &str,
    strategy_chirho: Option<DerivingStrategyChirho>,
) -> DerivedClassChirho {
    DerivedClassChirho::applied_chirho(
        var_name_chirho(class_chirho),
        vec![con_ty_chirho("Int")],
        strategy_chirho,
    )
}

fn newtype_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::newtype_at_chirho(gen_span_chirho()))
}

fn stock_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::stock_at_chirho(gen_span_chirho()))
}

fn anyclass_chirho() -> Option<DerivingStrategyChirho> {
    Some(DerivingStrategyChirho::anyclass_at_chirho(gen_span_chirho()))
}

/// `newtype <name> <vars> = <name> <field>`
fn newtype_decl_chirho(
    name_chirho: &str,
    vars_chirho: &[&str],
    field_chirho: TypeChirho,
    deriving_chirho: Vec<DerivedClassChirho>,
) -> DeclChirho {
    DeclChirho::NewtypeDeclChirho {
        name_chirho: var_name_chirho(name_chirho),
        type_vars_chirho: vars_chirho
            .iter()
            .map(|var_chirho| TyVarChirho::plain_chirho(var_name_chirho(var_chirho)))
            .collect(),
        constructor_chirho: ConDeclChirho::OrdinaryChirho {
            name_chirho: var_name_chirho(name_chirho),
            fields_chirho: vec![(StrictnessChirho::LazyChirho, field_chirho)],
            span_chirho: gen_span_chirho(),
            prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
        },
        deriving_chirho,
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    }
}

/// `newtype AgeChirho = AgeChirho Int` deriving `deriving_chirho`.
fn age_chirho(deriving_chirho: Vec<DerivedClassChirho>) -> DeclChirho {
    newtype_decl_chirho("AgeChirho", &[], con_ty_chirho("Int"), deriving_chirho)
}

/// Each generated instance's class, context and methods count.
fn instance_shapes_chirho(
    result_chirho: &DerivingResultChirho,
) -> Vec<(String, Vec<ConstraintChirho>, usize)> {
    result_chirho
        .instances_chirho
        .iter()
        .map(|instance_chirho| match instance_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                context_chirho,
                methods_chirho,
                ..
            } => (
                class_chirho.text_chirho().to_string(),
                context_chirho.clone(),
                methods_chirho.len(),
            ),
            other_chirho => panic!("expected an instance, got {other_chirho:?}"),
        })
        .collect()
}

fn show_int_context_chirho() -> Vec<ConstraintChirho> {
    vec![ConstraintChirho::ClassChirho {
        kind_args_chirho: Vec::new(),
        class_chirho: var_name_chirho("Show"),
        args_chirho: vec![con_ty_chirho("Int")],
        span_chirho: gen_span_chirho(),
    }]
}

#[test]
fn an_explicit_newtype_show_takes_the_representation_chirho() {
    let result_chirho =
        derive_instances_chirho(&module_chirho(vec![age_chirho(vec![derived_chirho(
            "Show",
            newtype_chirho(),
        )])]));
    assert!(
        result_chirho.warnings_chirho.is_empty(),
        "{:?}",
        result_chirho.warnings_chirho
    );
    // `Show Int => Show AgeChirho` with no written methods: never the stock rendering.
    assert_eq!(
        instance_shapes_chirho(&result_chirho),
        [("Show".to_string(), show_int_context_chirho(), 0)]
    );
}

#[test]
fn the_stock_twin_and_an_unwritten_show_render_the_constructor_chirho() {
    for strategy_chirho in [stock_chirho(), None] {
        let result_chirho = derive_instances_with_builtins_chirho(
            &module_chirho(vec![age_chirho(vec![derived_chirho(
                "Show",
                strategy_chirho.clone(),
            )])]),
            &prelude_scope_chirho(&[CompilerClassChirho::ShowChirho]),
        );
        let shapes_chirho = instance_shapes_chirho(&result_chirho);
        assert_eq!(shapes_chirho.len(), 1, "{strategy_chirho:?}");
        let (class_chirho, context_chirho, methods_chirho) = &shapes_chirho[0];
        assert_eq!(class_chirho, "Show");
        assert!(
            context_chirho.is_empty(),
            "{strategy_chirho:?}: {context_chirho:?}"
        );
        assert!(*methods_chirho > 0, "the stock Show writes its methods");
    }
}

#[test]
fn an_explicit_anyclass_writes_an_empty_instance_chirho() {
    let result_chirho =
        derive_instances_chirho(&module_chirho(vec![age_chirho(vec![derived_chirho(
            "Show",
            anyclass_chirho(),
        )])]));
    assert_eq!(
        instance_shapes_chirho(&result_chirho),
        [("Show".to_string(), vec![], 0)]
    );
}

#[test]
fn the_newtype_strategy_on_data_derives_nothing_and_leaves_the_refusal_to_validity_chirho() {
    let data_chirho = DeclChirho::DataDeclChirho {
        name_chirho: var_name_chirho("BadChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: var_name_chirho("BadChirho"),
            fields_chirho: vec![(StrictnessChirho::LazyChirho, con_ty_chirho("Int"))],
            span_chirho: gen_span_chirho(),
            prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
        }],
        deriving_chirho: vec![derived_chirho("Show", newtype_chirho())],
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    };
    let result_chirho = derive_instances_chirho(&module_chirho(vec![data_chirho]));
    assert!(result_chirho.instances_chirho.is_empty());
    assert!(result_chirho.warnings_chirho.is_empty());
}

/// `newtype DBWith b e a = DB (DBImpl b e a)` deriving newtype Functor (T25266's shape).
fn db_with_chirho() -> ModuleChirho {
    let representation_chirho = app_ty_chirho(
        app_ty_chirho(
            app_ty_chirho(con_ty_chirho("DBImpl"), var_ty_chirho("b")),
            var_ty_chirho("e"),
        ),
        var_ty_chirho("a"),
    );
    module_chirho(vec![newtype_decl_chirho(
        "DBWith",
        &["b", "e", "a"],
        TypeChirho::ParenChirho {
            inner_chirho: Box::new(representation_chirho),
            span_chirho: gen_span_chirho(),
        },
        vec![derived_chirho("Functor", newtype_chirho())],
    )])
}

/// A scope proving each class's bare spelling, as the implicit Prelude import does.
fn prelude_scope_chirho(classes_chirho: &[CompilerClassChirho]) -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(
        classes_chirho
            .iter()
            .map(|&class_chirho| {
                (
                    class_chirho.legacy_name_chirho().to_string(),
                    BuiltinClassChirho {
                        identity_chirho: class_chirho,
                        module_chirho: "Prelude",
                        name_chirho: class_chirho.legacy_name_chirho(),
                    },
                )
            })
            .collect(),
    )
}

/// The single instance's head type and context argument.
fn gnd_head_and_context_chirho(result_chirho: &DerivingResultChirho) -> (TypeChirho, TypeChirho) {
    let [
        DeclChirho::InstanceDeclChirho {
            context_chirho,
            types_chirho,
            ..
        },
    ] = result_chirho.instances_chirho.as_slice()
    else {
        panic!(
            "expected one instance, got {:?}",
            result_chirho.instances_chirho
        );
    };
    let [ConstraintChirho::ClassChirho { args_chirho, .. }] = context_chirho.as_slice() else {
        panic!("expected one class constraint, got {context_chirho:?}");
    };
    (types_chirho[0].clone(), args_chirho[0].clone())
}

#[test]
fn a_gnd_head_drops_the_variables_a_constructor_class_needs_chirho() {
    let result_chirho = derive_instances_with_builtins_chirho(
        &db_with_chirho(),
        &prelude_scope_chirho(&[CompilerClassChirho::FunctorChirho]),
    );
    let (head_chirho, representation_chirho) = gnd_head_and_context_chirho(&result_chirho);
    // `Functor (DBImpl b e) => Functor (DBWith b e)`
    assert_eq!(
        head_chirho,
        app_ty_chirho(
            app_ty_chirho(con_ty_chirho("DBWith"), var_ty_chirho("b")),
            var_ty_chirho("e"),
        )
    );
    assert_eq!(
        representation_chirho,
        app_ty_chirho(
            app_ty_chirho(con_ty_chirho("DBImpl"), var_ty_chirho("b")),
            var_ty_chirho("e"),
        )
    );
}

#[test]
fn without_the_proved_class_a_gnd_head_keeps_every_variable_chirho() {
    let result_chirho = derive_instances_chirho(&db_with_chirho());
    let (head_chirho, _) = gnd_head_and_context_chirho(&result_chirho);
    assert_eq!(
        head_chirho,
        app_ty_chirho(
            app_ty_chirho(
                app_ty_chirho(con_ty_chirho("DBWith"), var_ty_chirho("b")),
                var_ty_chirho("e"),
            ),
            var_ty_chirho("a"),
        )
    );
}

#[test]
fn a_representation_that_cannot_drop_the_variable_derives_nothing_chirho() {
    // `newtype Phantom a = Phantom Int deriving newtype Functor`: `Int` does not end in `a`.
    let module_chirho = module_chirho(vec![newtype_decl_chirho(
        "Phantom",
        &["a"],
        con_ty_chirho("Int"),
        vec![derived_chirho("Functor", newtype_chirho())],
    )]);
    let result_chirho = derive_instances_with_builtins_chirho(
        &module_chirho,
        &prelude_scope_chirho(&[CompilerClassChirho::FunctorChirho]),
    );
    assert!(result_chirho.instances_chirho.is_empty());
    assert_eq!(result_chirho.warnings_chirho.len(), 1);
}

#[test]
fn a_gnd_head_drops_the_two_variables_a_transformer_class_needs_chirho() {
    // T12734: `newtype KnownTyx cls t m a = KnownTyx (IdentityT m a) deriving newtype
    // MonadTrans` asks for `MonadTrans IdentityT => MonadTrans (KnownTyx cls t)`.
    let field_chirho = app_ty_chirho(
        app_ty_chirho(con_ty_chirho("IdentityT"), var_ty_chirho("m")),
        var_ty_chirho("a"),
    );
    let module_chirho = module_chirho(vec![newtype_decl_chirho(
        "KnownTyx",
        &["cls", "t", "m", "a"],
        TypeChirho::ParenChirho {
            inner_chirho: Box::new(field_chirho),
            span_chirho: gen_span_chirho(),
        },
        vec![derived_chirho("MonadTrans", newtype_chirho())],
    )]);
    let result_chirho = derive_instances_with_builtins_chirho(
        &module_chirho,
        &prelude_scope_chirho(&[CompilerClassChirho::MonadTransChirho]),
    );
    let (head_chirho, representation_chirho) = gnd_head_and_context_chirho(&result_chirho);
    assert_eq!(
        head_chirho,
        app_ty_chirho(
            app_ty_chirho(con_ty_chirho("KnownTyx"), var_ty_chirho("cls")),
            var_ty_chirho("t"),
        )
    );
    assert_eq!(representation_chirho, con_ty_chirho("IdentityT"));
}

#[test]
fn only_newtype_routed_classes_take_the_representation_dictionary_chirho() {
    let standalone_chirho = DeclChirho::StandaloneDerivingDeclChirho {
        strategy_chirho: newtype_chirho(),
        context_chirho: vec![],
        class_chirho: var_name_chirho("Ord"),
        types_chirho: vec![con_ty_chirho("AgeChirho")],
        span_chirho: gen_span_chirho(),
    };
    let classes_chirho = newtype_strategy_classes_chirho(
        &module_chirho(vec![
            age_chirho(vec![
                derived_chirho("Eq", stock_chirho()),
                derived_chirho("Show", newtype_chirho()),
                derived_chirho("Read", None),
                derived_chirho("Num", None),
                derived_chirho("Default", anyclass_chirho()),
                // A class with arguments never takes the representation's dictionary.
                applied_chirho("ScaleChirho", newtype_chirho()),
            ]),
            standalone_chirho,
        ]),
        &BuiltinClassScopeChirho::default(),
    );
    assert_eq!(
        classes_chirho,
        HashMap::from([(
            "AgeChirho".to_string(),
            vec!["Show".to_string(), "Num".to_string(), "Ord".to_string()],
        )])
    );
}

#[test]
fn an_anyclass_show_has_no_constructor_rendering_chirho() {
    let data_chirho = |strategy_chirho| DeclChirho::DataDeclChirho {
        name_chirho: var_name_chirho("ColorChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: var_name_chirho("RedChirho"),
            fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
            prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
        }],
        deriving_chirho: vec![derived_chirho("Show", strategy_chirho)],
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    };
    let prelude_chirho = prelude_scope_chirho(&[CompilerClassChirho::ShowChirho]);
    assert!(
        derived_show_metadata_with_builtins_chirho(
            &module_chirho(vec![data_chirho(stock_chirho())]),
            &prelude_chirho
        )
        .contains_key("ColorChirho")
    );
    assert!(
        !derived_show_metadata_with_builtins_chirho(
            &module_chirho(vec![data_chirho(anyclass_chirho())]),
            &prelude_chirho
        )
        .contains_key("ColorChirho")
    );
    // Without the proof an explicit `stock` names no stock class (#31786).
    assert!(
        !derived_show_metadata_chirho(&module_chirho(vec![data_chirho(stock_chirho())]))
            .contains_key("ColorChirho")
    );
}

#[test]
fn derive_any_class_takes_an_unwritten_newtype_class_over_gnd_chirho() {
    // Root's D1 (GHC 9.14.1): with DeriveAnyClass and GND both on, an unwritten strategy is
    // anyclass, with warning GHC-20042. D2: without DeriveAnyClass it is GND.
    let mut program_chirho = module_chirho(vec![age_chirho(vec![derived_chirho(
        "DescribeChirho",
        None,
    )])]);
    program_chirho.extensions_chirho = vec!["DeriveAnyClass".to_string()];
    let anyclass_chirho = derive_instances_chirho(&program_chirho);
    assert_eq!(
        instance_shapes_chirho(&anyclass_chirho),
        [("DescribeChirho".to_string(), vec![], 0)]
    );
    assert!(
        anyclass_chirho
            .warnings_chirho
            .iter()
            .any(|warning_chirho| warning_chirho.contains("GHC-20042")),
        "{:?}",
        anyclass_chirho.warnings_chirho
    );
    program_chirho.extensions_chirho.clear();
    let gnd_chirho = derive_instances_chirho(&program_chirho);
    assert_eq!(
        instance_shapes_chirho(&gnd_chirho),
        [(
            "DescribeChirho".to_string(),
            vec![ConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
                class_chirho: var_name_chirho("DescribeChirho"),
                args_chirho: vec![con_ty_chirho("Int")],
                span_chirho: gen_span_chirho(),
            }],
            0
        )]
    );
    assert!(gnd_chirho.warnings_chirho.is_empty());
}

/// `data <name> <vars> = <name>` deriving `deriving_chirho`.
fn data_decl_chirho(
    name_chirho: &str,
    vars_chirho: &[&str],
    deriving_chirho: Vec<DerivedClassChirho>,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: var_name_chirho(name_chirho),
        type_vars_chirho: vars_chirho
            .iter()
            .map(|var_chirho| TyVarChirho::plain_chirho(var_name_chirho(var_chirho)))
            .collect(),
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: var_name_chirho(name_chirho),
            fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
            prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
        }],
        deriving_chirho,
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    }
}

/// Each generated instance's class, as written, and head types.
fn instance_heads_chirho(result_chirho: &DerivingResultChirho) -> Vec<(String, Vec<TypeChirho>)> {
    result_chirho
        .instances_chirho
        .iter()
        .map(|instance_chirho| match instance_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                types_chirho,
                ..
            } => (class_chirho.full_name_chirho(), types_chirho.clone()),
            other_chirho => panic!("expected an instance, got {other_chirho:?}"),
        })
        .collect()
}

#[test]
fn an_anyclass_instance_takes_the_full_written_head_chirho() {
    // W1 (GHC prints `paired`): `deriving anyclass (PairChirho Int)` is `PairChirho Int TagChirho`.
    let mut module_chirho = module_chirho(vec![
        data_decl_chirho(
            "TagChirho",
            &[],
            vec![applied_chirho("PairChirho", anyclass_chirho())],
        ),
        data_decl_chirho(
            "BoxChirho",
            &["a"],
            vec![derived_chirho("MarkChirho", anyclass_chirho())],
        ),
    ]);
    module_chirho.extensions_chirho = vec![
        "DerivingStrategies".to_string(),
        "DeriveAnyClass".to_string(),
    ];
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert_eq!(
        instance_heads_chirho(&result_chirho),
        [
            (
                "PairChirho".to_string(),
                vec![con_ty_chirho("Int"), con_ty_chirho("TagChirho")]
            ),
            // A parameterized type is one applied head type, never `MarkChirho BoxChirho a`.
            (
                "MarkChirho".to_string(),
                vec![app_ty_chirho(
                    con_ty_chirho("BoxChirho"),
                    var_ty_chirho("a")
                )]
            ),
        ]
    );
    assert!(result_chirho.warnings_chirho.is_empty());
}

/// A scope proving `Typeable` as `import Data.Typeable (Typeable)` does.
fn typeable_scope_chirho() -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(HashMap::from([(
        "Typeable".to_string(),
        BuiltinClassChirho {
            identity_chirho: CompilerClassChirho::TypeableChirho,
            module_chirho: "Data.Typeable",
            name_chirho: "Typeable",
        },
    )]))
}

#[test]
fn a_proven_builtin_typeable_request_derives_nothing_under_any_strategy_chirho() {
    // GHC 9.14.1 ignores it (#32106; typeable-route r2, r6 and r7): no instance, no warning and
    // no representation dictionary, so the stock Show beside it keeps the constructor.
    let standalone_chirho = DeclChirho::StandaloneDerivingDeclChirho {
        strategy_chirho: newtype_chirho(),
        context_chirho: vec![],
        class_chirho: var_name_chirho("Typeable"),
        types_chirho: vec![con_ty_chirho("AgeChirho")],
        span_chirho: gen_span_chirho(),
    };
    for extensions_chirho in [
        &[][..],
        &[
            "DerivingStrategies",
            "GeneralizedNewtypeDeriving",
            "DeriveAnyClass",
        ][..],
    ] {
        let mut module_chirho = module_chirho(vec![
            age_chirho(vec![
                derived_chirho("Show", None),
                derived_chirho("Typeable", None),
                derived_chirho("Typeable", stock_chirho()),
                derived_chirho("Typeable", newtype_chirho()),
                derived_chirho("Typeable", anyclass_chirho()),
            ]),
            data_decl_chirho("TagChirho", &[], vec![derived_chirho("Typeable", None)]),
            standalone_chirho.clone(),
        ]);
        module_chirho.extensions_chirho = extensions_chirho
            .iter()
            .map(|extension_chirho| extension_chirho.to_string())
            .collect();
        let result_chirho =
            derive_instances_with_builtins_chirho(&module_chirho, &typeable_scope_chirho());
        assert!(
            result_chirho.warnings_chirho.is_empty(),
            "{extensions_chirho:?}: {:?}",
            result_chirho.warnings_chirho
        );
        let shapes_chirho = instance_shapes_chirho(&result_chirho);
        assert_eq!(
            shapes_chirho.len(),
            1,
            "{extensions_chirho:?}: only the stock Show"
        );
        let (class_chirho, context_chirho, methods_chirho) = &shapes_chirho[0];
        assert_eq!(class_chirho, "Show");
        assert!(
            context_chirho.is_empty() && *methods_chirho > 0,
            "{extensions_chirho:?}: the stock Show writes its methods"
        );
        assert!(
            newtype_strategy_classes_chirho(&module_chirho, &typeable_scope_chirho()).is_empty(),
            "{extensions_chirho:?}"
        );
    }
}

#[test]
fn only_the_proven_builtin_typeable_is_ignored_chirho() {
    // TypeableLocalGndChirho's shape: the module's own `class Typeable` keeps genuine GND, as
    // GHC 9.14.1 does (it prints `int`, typeable-route r2). K01 never proves a local class,
    // so the scope is empty, and a spelling K01 cannot judge keeps its route too.
    let local_class_chirho = DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![],
        name_chirho: var_name_chirho("Typeable"),
        type_vars_chirho: vec![var_name_chirho("a").into()],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    for decls_chirho in [
        vec![
            local_class_chirho,
            age_chirho(vec![derived_chirho("Typeable", None)]),
        ],
        vec![age_chirho(vec![derived_chirho("Typeable", None)])],
    ] {
        let module_chirho = module_chirho(decls_chirho);
        let scope_chirho = BuiltinClassScopeChirho::default();
        let result_chirho = derive_instances_with_builtins_chirho(&module_chirho, &scope_chirho);
        assert_eq!(
            instance_shapes_chirho(&result_chirho),
            [(
                "Typeable".to_string(),
                vec![ConstraintChirho::ClassChirho {
                    kind_args_chirho: Vec::new(),
                    class_chirho: var_name_chirho("Typeable"),
                    args_chirho: vec![con_ty_chirho("Int")],
                    span_chirho: gen_span_chirho(),
                }],
                0
            )]
        );
        assert_eq!(
            newtype_strategy_classes_chirho(&module_chirho, &scope_chirho),
            HashMap::from([("AgeChirho".to_string(), vec!["Typeable".to_string()])])
        );
    }
}

#[test]
fn a_class_with_arguments_derives_nothing_through_stock_or_gnd_chirho() {
    // Declaration validity refuses both by name (GHC-73993; W3's Unsupported gap).
    let mut module_chirho = module_chirho(vec![
        data_decl_chirho("TagChirho", &[], vec![applied_chirho("Eq", stock_chirho())]),
        age_chirho(vec![
            applied_chirho("ScaleChirho", newtype_chirho()),
            applied_chirho("ScaleChirho", None),
        ]),
    ]);
    module_chirho.extensions_chirho = vec!["DerivingStrategies".to_string()];
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert!(
        result_chirho.instances_chirho.is_empty(),
        "{:?}",
        result_chirho.instances_chirho
    );
    assert!(result_chirho.warnings_chirho.is_empty());
}

#[test]
fn a_stock_instance_names_its_class_as_written_chirho() {
    // S2 (GHC prints `same`): `P.Eq` beside a local `class Eq` is the Prelude's class.
    let builtin_chirho = BuiltinClassScopeChirho::new_chirho(HashMap::from([(
        "P.Eq".to_string(),
        BuiltinClassChirho {
            identity_chirho: CompilerClassChirho::EqChirho,
            module_chirho: "GHC.Classes",
            name_chirho: "Eq",
        },
    )]));
    let local_eq_chirho = DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![],
        name_chirho: var_name_chirho("Eq"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(var_name_chirho("valueChirho"))],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    let prelude_eq_chirho = DerivedClassChirho::new_chirho(
        NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            "P",
            "Eq",
            gen_span_chirho(),
        )),
        stock_chirho(),
    );
    let mut module_chirho = module_chirho(vec![
        local_eq_chirho,
        data_decl_chirho("BoxChirho", &["a"], vec![prelude_eq_chirho]),
        // S1: the local class is not the builtin one, so no generator derives it.
        data_decl_chirho(
            "ColorChirho",
            &[],
            vec![derived_chirho("Eq", stock_chirho())],
        ),
    ]);
    module_chirho.extensions_chirho = vec!["DerivingStrategies".to_string()];
    let result_chirho = derive_instances_with_builtins_chirho(&module_chirho, &builtin_chirho);
    let [
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            context_chirho,
            methods_chirho,
            ..
        },
    ] = result_chirho.instances_chirho.as_slice()
    else {
        panic!(
            "expected only P.Eq's instance, got {:?}",
            result_chirho.instances_chirho
        );
    };
    assert_eq!(class_chirho.full_name_chirho(), "P.Eq");
    assert!(!methods_chirho.is_empty());
    let context_classes_chirho: Vec<String> = context_chirho
        .iter()
        .map(|constraint_chirho| match constraint_chirho {
            ConstraintChirho::ClassChirho { class_chirho, .. } => class_chirho.full_name_chirho(),
            other_chirho => format!("{other_chirho:?}"),
        })
        .collect();
    assert_eq!(context_classes_chirho, ["P.Eq"]);
}

#[test]
fn a_standalone_head_derives_for_its_last_type_chirho() {
    let standalone_chirho = DeclChirho::StandaloneDerivingDeclChirho {
        strategy_chirho: anyclass_chirho(),
        context_chirho: vec![],
        class_chirho: var_name_chirho("PairChirho"),
        types_chirho: vec![con_ty_chirho("Int"), con_ty_chirho("TagChirho")],
        span_chirho: gen_span_chirho(),
    };
    let mut module_chirho = module_chirho(vec![
        data_decl_chirho("TagChirho", &[], vec![]),
        standalone_chirho,
    ]);
    module_chirho.extensions_chirho = vec![
        "DerivingStrategies".to_string(),
        "DeriveAnyClass".to_string(),
    ];
    let result_chirho = derive_instances_chirho(&module_chirho);
    assert_eq!(
        instance_heads_chirho(&result_chirho),
        [(
            "PairChirho".to_string(),
            vec![con_ty_chirho("Int"), con_ty_chirho("TagChirho")]
        )]
    );
    assert!(result_chirho.warnings_chirho.is_empty());
}
