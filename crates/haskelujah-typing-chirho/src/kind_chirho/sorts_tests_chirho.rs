// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Module-level sort judgments (K01 reject frontier). The fixtures mirror the GHC
//! oracle set `oracle-sorts-chirho` (P01-P10 and N01-N08). Their verdicts are
//! predictions until GHC 9.14.1 measures that set.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{ClassMethodChirho, ConDeclChirho, StrictnessChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

const TYPE_POSITION_ERROR_CHIRHO: &str = "is a constraint";
const CONSTRAINT_POSITION_ERROR_CHIRHO: &str = "is a type";

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
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

fn paren_chirho(inner_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::ParenChirho {
        inner_chirho: Box::new(inner_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn tuple_chirho(elements_chirho: Vec<TypeChirho>) -> TypeChirho {
    TypeChirho::TupleChirho {
        elements_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `(left op right)`, the shape the parser lowers an infix type operator to.
fn infix_chirho(
    operator_chirho: &str,
    left_chirho: TypeChirho,
    right_chirho: TypeChirho,
) -> TypeChirho {
    paren_chirho(app_chirho(
        app_chirho(con_chirho(operator_chirho), left_chirho),
        right_chirho,
    ))
}

fn constraint_chirho(class_text_chirho: &str, args_chirho: Vec<TypeChirho>) -> ConstraintChirho {
    ConstraintChirho::ClassChirho {
        class_chirho: name_chirho(class_text_chirho),
        args_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn qualified_chirho(context_chirho: Vec<ConstraintChirho>, body_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::QualChirho {
        context_chirho,
        body_chirho: Box::new(body_chirho),
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

/// `class <name> a where <method> :: <method type>`.
fn class_chirho(
    class_text_chirho: &str,
    method_text_chirho: &str,
    method_ty_chirho: TypeChirho,
) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho(class_text_chirho),
        type_vars_chirho: vec![name_chirho("a").into()],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: name_chirho(method_text_chirho),
            ty_chirho: method_ty_chirho,
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `class TagChirho a where tagChirho :: a -> a`, of kind `* -> Constraint`.
fn tag_class_chirho() -> DeclChirho {
    class_chirho(
        "TagChirho",
        "tagChirho",
        fun_chirho(var_chirho("a"), var_chirho("a")),
    )
}

/// `data <name> <binders> = <constructor> <fields>`.
fn data_chirho(
    type_text_chirho: &str,
    binders_chirho: Vec<TyVarChirho>,
    fields_chirho: Vec<TypeChirho>,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_text_chirho),
        type_vars_chirho: binders_chirho,
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

/// `data DictChirho (c :: Constraint) = MkDictChirho`.
fn dict_chirho() -> DeclChirho {
    data_chirho(
        "DictChirho",
        vec![TyVarChirho::annotated_chirho(
            name_chirho("c"),
            AstKindChirho::ConstraintChirho,
        )],
        vec![],
    )
}

fn synonym_chirho(synonym_text_chirho: &str, rhs_chirho: TypeChirho) -> DeclChirho {
    DeclChirho::TypeAliasDeclChirho {
        name_chirho: name_chirho(synonym_text_chirho),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("a"))],
        rhs_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `type family BothChirho a :: Constraint`.
fn constraint_family_chirho() -> DeclChirho {
    DeclChirho::TypeFamilyDeclChirho {
        name_chirho: name_chirho("BothChirho"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("a"))],
        result_kind_chirho: Some(con_chirho("Constraint")),
        data_family_chirho: false,
        standalone_kind_chirho: None,
        closed_chirho: false,
        equations_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// Error count and rendered diagnostics of one module's kind pass.
fn kind_errors_chirho(decls_chirho: Vec<DeclChirho>) -> (usize, String) {
    let module_chirho = ModuleChirho {
        name_chirho: name_chirho("SortsChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    (
        result_chirho.diagnostics_chirho.error_count_chirho(),
        result_chirho.diagnostics_chirho.to_string(),
    )
}

#[test]
fn a_local_class_written_as_a_field_is_reported_chirho() {
    // N01, tcfail146's shape: `data SDataChirho a = MkSDataChirho (SClassChirho a)`.
    let (count_chirho, text_chirho) = kind_errors_chirho(vec![
        class_chirho(
            "SClassChirho",
            "sFunChirho",
            fun_chirho(var_chirho("a"), var_chirho("a")),
        ),
        data_chirho(
            "SDataChirho",
            vec![TyVarChirho::plain_chirho(name_chirho("a"))],
            vec![paren_chirho(app_chirho(
                con_chirho("SClassChirho"),
                var_chirho("a"),
            ))],
        ),
    ]);
    assert_eq!(count_chirho, 1, "{text_chirho}");
    assert_eq!(text_chirho.matches(TYPE_POSITION_ERROR_CHIRHO).count(), 1);
}

#[test]
fn equality_where_a_type_is_required_is_reported_once_per_position_chirho() {
    // N02-N04, T3540's shapes, plus a class method whose whole type is an equality.
    let equality_chirho = || infix_chirho("~", var_chirho("a"), con_chirho("Int"));
    let (count_chirho, text_chirho) = kind_errors_chirho(vec![
        signature_chirho("thingChirho", equality_chirho()),
        signature_chirho(
            "thingOneChirho",
            fun_chirho(con_chirho("Int"), equality_chirho()),
        ),
        signature_chirho(
            "thingTwoChirho",
            fun_chirho(equality_chirho(), con_chirho("Int")),
        ),
        class_chirho("SameChirho", "sameChirho", equality_chirho()),
    ]);
    assert_eq!(count_chirho, 4, "{text_chirho}");
    assert_eq!(text_chirho.matches(TYPE_POSITION_ERROR_CHIRHO).count(), 4);
}

#[test]
fn constraint_synonyms_and_families_where_a_type_is_required_are_reported_chirho() {
    // N07 and N08: a local constraint-tuple synonym and a Constraint-result family.
    let pair_chirho = app_chirho(con_chirho("TagChirho"), var_chirho("a"));
    let (count_chirho, text_chirho) = kind_errors_chirho(vec![
        tag_class_chirho(),
        synonym_chirho(
            "TagPairChirho",
            tuple_chirho(vec![pair_chirho.clone(), pair_chirho]),
        ),
        constraint_family_chirho(),
        signature_chirho(
            "usePairChirho",
            fun_chirho(
                app_chirho(con_chirho("TagPairChirho"), var_chirho("a")),
                con_chirho("Int"),
            ),
        ),
        signature_chirho(
            "useBothChirho",
            fun_chirho(
                app_chirho(con_chirho("BothChirho"), con_chirho("Int")),
                con_chirho("Int"),
            ),
        ),
    ]);
    assert_eq!(count_chirho, 2, "{text_chirho}");
    assert_eq!(text_chirho.matches(TYPE_POSITION_ERROR_CHIRHO).count(), 2);
}

#[test]
fn a_local_data_type_written_as_a_constraint_is_reported_chirho() {
    // N06, T13677's shape: `fooChirho :: DictChirho (Int ~ Int) => Int`.
    let (count_chirho, text_chirho) = kind_errors_chirho(vec![
        dict_chirho(),
        signature_chirho(
            "fooChirho",
            qualified_chirho(
                vec![constraint_chirho(
                    "DictChirho",
                    vec![infix_chirho("~", con_chirho("Int"), con_chirho("Int"))],
                )],
                con_chirho("Int"),
            ),
        ),
    ]);
    assert_eq!(count_chirho, 1, "{text_chirho}");
    assert_eq!(
        text_chirho
            .matches(CONSTRAINT_POSITION_ERROR_CHIRHO)
            .count(),
        1
    );
}

#[test]
fn constraints_where_a_constraint_is_expected_hold_chirho() {
    // P01, P02, P07 and P08, plus `~` at kinds `*` and `* -> *` in one
    // signature (fresh per use), an imported constructor reused at two kinds,
    // and heterogeneous `~~`.
    let dict_of_chirho = |argument_chirho| app_chirho(con_chirho("DictChirho"), argument_chirho);
    let (count_chirho, text_chirho) = kind_errors_chirho(vec![
        dict_chirho(),
        tag_class_chirho(),
        data_chirho(
            "ProxyChirho",
            vec![TyVarChirho::plain_chirho(name_chirho("p"))],
            vec![],
        ),
        signature_chirho(
            "showWithChirho",
            fun_chirho(
                dict_of_chirho(paren_chirho(app_chirho(
                    con_chirho("Show"),
                    var_chirho("a"),
                ))),
                fun_chirho(var_chirho("a"), con_chirho("String")),
            ),
        ),
        signature_chirho(
            "proxyChirho",
            app_chirho(
                con_chirho("ProxyChirho"),
                paren_chirho(app_chirho(con_chirho("TagChirho"), con_chirho("Int"))),
            ),
        ),
        signature_chirho(
            "useChirho",
            fun_chirho(
                dict_of_chirho(paren_chirho(app_chirho(
                    con_chirho("TagChirho"),
                    var_chirho("a"),
                ))),
                fun_chirho(var_chirho("a"), con_chirho("String")),
            ),
        ),
        signature_chirho(
            "sameIntChirho",
            qualified_chirho(
                vec![constraint_chirho(
                    "~",
                    vec![var_chirho("a"), con_chirho("Int")],
                )],
                fun_chirho(var_chirho("a"), var_chirho("a")),
            ),
        ),
        signature_chirho(
            "bothChirho",
            fun_chirho(
                dict_of_chirho(infix_chirho("~", var_chirho("a"), con_chirho("Int"))),
                fun_chirho(
                    dict_of_chirho(infix_chirho("~", var_chirho("f"), con_chirho("Maybe"))),
                    con_chirho("Int"),
                ),
            ),
        ),
        // An imported constructor first used at `Constraint` keeps that kind,
        // so `a` resolves to `Constraint`; a variable head is never judged.
        signature_chirho(
            "importedChirho",
            fun_chirho(
                app_chirho(
                    con_chirho("ImportedProxyChirho"),
                    paren_chirho(app_chirho(con_chirho("TagChirho"), con_chirho("Int"))),
                ),
                fun_chirho(
                    app_chirho(con_chirho("ImportedProxyChirho"), var_chirho("a")),
                    var_chirho("a"),
                ),
            ),
        ),
        signature_chirho(
            "heteroChirho",
            fun_chirho(
                dict_of_chirho(infix_chirho("~~", var_chirho("a"), con_chirho("Maybe"))),
                con_chirho("Int"),
            ),
        ),
    ]);
    assert_eq!(count_chirho, 0, "{text_chirho}");
}

#[test]
fn constraint_synonyms_families_and_unit_hold_chirho() {
    // P03/P09, P04 and P06 in contexts, plus `()` in a type position.
    let a_chirho = || var_chirho("a");
    let to_string_chirho = || fun_chirho(var_chirho("a"), con_chirho("String"));
    let unit_chirho = || tuple_chirho(vec![]);
    let (count_chirho, text_chirho) = kind_errors_chirho(vec![
        tag_class_chirho(),
        synonym_chirho(
            "TaggedChirho",
            tuple_chirho(vec![
                app_chirho(con_chirho("TagChirho"), a_chirho()),
                app_chirho(con_chirho("Show"), a_chirho()),
            ]),
        ),
        synonym_chirho(
            "PrintableChirho",
            app_chirho(con_chirho("Show"), a_chirho()),
        ),
        constraint_family_chirho(),
        signature_chirho(
            "describeChirho",
            qualified_chirho(
                vec![constraint_chirho("TaggedChirho", vec![a_chirho()])],
                to_string_chirho(),
            ),
        ),
        signature_chirho(
            "renderChirho",
            qualified_chirho(
                vec![constraint_chirho("PrintableChirho", vec![a_chirho()])],
                to_string_chirho(),
            ),
        ),
        signature_chirho(
            "sameChirho",
            qualified_chirho(
                vec![constraint_chirho("BothChirho", vec![a_chirho()])],
                fun_chirho(a_chirho(), to_string_chirho()),
            ),
        ),
        signature_chirho("unitChirho", fun_chirho(con_chirho("Int"), unit_chirho())),
        signature_chirho("mainChirho", app_chirho(con_chirho("IO"), unit_chirho())),
    ]);
    assert_eq!(count_chirho, 0, "{text_chirho}");
}
