// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving strategy routes (row 694): an explicit `newtype` takes the representation's
//! instance and never a stock one, `stock` and `anyclass` stay themselves, an unwritten
//! strategy keeps the legacy policy, and a GND head drops the variables a builtin class
//! over `Type -> Type` needs.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    DerivedClassChirho, DerivingStrategyChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::name_chirho::BuiltinClassChirho;
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
        let result_chirho =
            derive_instances_chirho(&module_chirho(vec![age_chirho(vec![derived_chirho(
                "Show",
                strategy_chirho.clone(),
            )])]));
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

fn functor_scope_chirho() -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(HashMap::from([(
        "Functor".to_string(),
        BuiltinClassChirho {
            module_chirho: "GHC.Base",
            name_chirho: "Functor",
            parameter_arities_chirho: &[1],
        },
    )]))
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
    let result_chirho =
        derive_instances_with_builtins_chirho(&db_with_chirho(), &functor_scope_chirho());
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
    let result_chirho =
        derive_instances_with_builtins_chirho(&module_chirho, &functor_scope_chirho());
    assert!(result_chirho.instances_chirho.is_empty());
    assert_eq!(result_chirho.warnings_chirho.len(), 1);
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
    let classes_chirho = newtype_strategy_classes_chirho(&module_chirho(vec![
        age_chirho(vec![
            derived_chirho("Eq", stock_chirho()),
            derived_chirho("Show", newtype_chirho()),
            derived_chirho("Read", None),
            derived_chirho("Num", None),
            derived_chirho("Default", anyclass_chirho()),
        ]),
        standalone_chirho,
    ]));
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
        }],
        deriving_chirho: vec![derived_chirho("Show", strategy_chirho)],
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    };
    assert!(
        derived_show_metadata_chirho(&module_chirho(vec![data_chirho(stock_chirho())]))
            .contains_key("ColorChirho")
    );
    assert!(
        !derived_show_metadata_chirho(&module_chirho(vec![data_chirho(anyclass_chirho())]))
            .contains_key("ColorChirho")
    );
}
