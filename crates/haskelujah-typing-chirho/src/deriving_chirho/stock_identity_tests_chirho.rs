// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Stock class identity (row 694, #31771): K01's proof names the builtin class whatever its
//! qualifier, a local declaration is never a builtin stock class, and a spelling K01 cannot
//! judge keeps its written name. Generated instances take the written spelling.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::DerivingStrategyChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use super::*;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn qualified_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `class <name> <parameters>` with no methods.
fn class_decl_chirho(class_chirho: &str, parameters_chirho: &[&str]) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho(class_chirho),
        type_vars_chirho: parameters_chirho
            .iter()
            .map(|parameter_chirho| TyVarChirho::plain_chirho(name_chirho(parameter_chirho)))
            .collect(),
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Main"),
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

/// A scope proving each spelling for its builtin class, all over one `Type` parameter.
fn scope_chirho(spellings_chirho: &[(&str, &'static str)]) -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(
        spellings_chirho
            .iter()
            .map(|&(spelling_chirho, class_chirho)| {
                (
                    spelling_chirho.to_string(),
                    BuiltinClassChirho {
                        module_chirho: "GHC.Classes",
                        name_chirho: class_chirho,
                        parameter_arities_chirho: &[0],
                    },
                )
            })
            .collect(),
    )
}

#[test]
fn a_proven_spelling_names_its_builtin_class_whatever_its_qualifier_chirho() {
    // `import qualified Prelude as P` beside a local `class Eq` (the S2 control's module).
    let builtin_chirho = scope_chirho(&[("P.Eq", "Eq"), ("P.Num", "Num")]);
    let module_chirho = module_chirho(vec![class_decl_chirho("Eq", &["valueChirho"])]);
    let identity_chirho = StockIdentityChirho::of_module_chirho(&module_chirho, &builtin_chirho);
    assert_eq!(
        identity_chirho.stock_class_chirho(&qualified_chirho("P", "Eq")),
        Some("Eq")
    );
    // A proven builtin class GHC does not derive with the stock strategy.
    assert_eq!(
        identity_chirho.stock_class_chirho(&qualified_chirho("P", "Num")),
        None
    );
    assert_eq!(
        identity_chirho.parameter_count_chirho(&qualified_chirho("P", "Num")),
        Some(1)
    );
}

#[test]
fn a_local_class_is_never_a_builtin_stock_class_chirho() {
    // The S1 control: `deriving stock (Eq)` beside `class Eq`.
    let builtin_chirho = scope_chirho(&[("P.Eq", "Eq")]);
    let module_chirho = module_chirho(vec![class_decl_chirho("Eq", &["valueChirho"])]);
    let identity_chirho = StockIdentityChirho::of_module_chirho(&module_chirho, &builtin_chirho);
    assert_eq!(identity_chirho.stock_class_chirho(&name_chirho("Eq")), None);
    assert_eq!(
        identity_chirho.stock_class_chirho(&qualified_chirho("Main", "Eq")),
        None
    );
    assert_eq!(
        identity_chirho.parameter_count_chirho(&name_chirho("Eq")),
        Some(1)
    );
}

#[test]
fn a_spelling_k01_cannot_judge_keeps_its_written_name_chirho() {
    let builtin_chirho = BuiltinClassScopeChirho::default();
    let module_chirho = module_chirho(vec![class_decl_chirho(
        "PairChirho",
        &["keyChirho", "valueChirho"],
    )]);
    let identity_chirho = StockIdentityChirho::of_module_chirho(&module_chirho, &builtin_chirho);
    for stock_chirho in ["Read", "Generic", "Show"] {
        assert_eq!(
            identity_chirho.stock_class_chirho(&name_chirho(stock_chirho)),
            Some(stock_chirho)
        );
    }
    for other_chirho in ["NFData", "DescribeChirho", "PairChirho"] {
        assert_eq!(
            identity_chirho.stock_class_chirho(&name_chirho(other_chirho)),
            None,
            "{other_chirho}"
        );
    }
    assert_eq!(
        identity_chirho.parameter_count_chirho(&name_chirho("PairChirho")),
        Some(2)
    );
    assert_eq!(
        identity_chirho.parameter_count_chirho(&name_chirho("Read")),
        Some(1)
    );
    // Only an interface knows an imported class's parameters.
    assert_eq!(
        identity_chirho.parameter_count_chirho(&name_chirho("DescribeChirho")),
        None
    );
}

#[test]
fn only_a_bare_stock_or_unwritten_entry_names_a_stock_class_chirho() {
    let builtin_chirho = BuiltinClassScopeChirho::default();
    let module_chirho = module_chirho(vec![]);
    let identity_chirho = StockIdentityChirho::of_module_chirho(&module_chirho, &builtin_chirho);
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let show_chirho = DerivedClassChirho::unspecified_chirho(name_chirho("Show"));
    assert!(identity_chirho.names_stock_class_chirho(&show_chirho, "Show"));
    assert!(!identity_chirho.names_stock_class_chirho(&show_chirho, "Eq"));
    assert!(!identity_chirho.names_stock_class_chirho(
        &DerivedClassChirho::new_chirho(
            name_chirho("Show"),
            Some(DerivingStrategyChirho::anyclass_at_chirho(span_chirho)),
        ),
        "Show"
    ));
    assert!(!identity_chirho.names_stock_class_chirho(
        &DerivedClassChirho::applied_chirho(
            name_chirho("Show"),
            vec![TypeChirho::ConChirho(name_chirho("Int"))],
            None,
        ),
        "Show"
    ));
}

#[test]
fn a_generated_instance_takes_the_written_class_spelling_chirho() {
    let constraint_chirho = |class_chirho: NameChirho| ConstraintChirho::ClassChirho {
        class_chirho,
        args_chirho: vec![TypeChirho::VarChirho(name_chirho("a"))],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut generated_chirho = vec![DeclChirho::InstanceDeclChirho {
        context_chirho: vec![
            constraint_chirho(name_chirho("Eq")),
            constraint_chirho(name_chirho("Show")),
        ],
        class_chirho: name_chirho("Eq"),
        types_chirho: vec![TypeChirho::ConChirho(name_chirho("ColorChirho"))],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }];
    spell_generated_class_chirho(&mut generated_chirho, "Eq", &qualified_chirho("P", "Eq"));
    let [
        DeclChirho::InstanceDeclChirho {
            class_chirho,
            context_chirho,
            ..
        },
    ] = generated_chirho.as_slice()
    else {
        panic!("one instance stays one instance");
    };
    assert_eq!(class_chirho.full_name_chirho(), "P.Eq");
    assert_eq!(
        context_chirho,
        &vec![
            constraint_chirho(qualified_chirho("P", "Eq")),
            constraint_chirho(name_chirho("Show")),
        ]
    );
}
