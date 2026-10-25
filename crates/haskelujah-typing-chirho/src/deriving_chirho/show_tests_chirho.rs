// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Derived-Show metadata (row 533): constructor forms, labels, precedences and the types
//! that are left out so their Show keeps its existing route.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{FieldDeclChirho, FixityChirho, StrictnessChirho};

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

fn data_chirho(
    name_chirho: &str,
    constructors_chirho: Vec<ConDeclChirho>,
    classes_chirho: &[&str],
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: var_name_chirho(name_chirho),
        type_vars_chirho: vec![],
        constructors_chirho,
        deriving_chirho: classes_chirho
            .iter()
            .map(|class_chirho| var_name_chirho(class_chirho))
            .collect(),
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    }
}

fn con_type_chirho(name_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(var_name_chirho(name_chirho))
}

fn ordinary_chirho(name_chirho: &str, fields_chirho: Vec<TypeChirho>) -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: var_name_chirho(name_chirho),
        fields_chirho: fields_chirho
            .into_iter()
            .map(|ty_chirho| (StrictnessChirho::LazyChirho, ty_chirho))
            .collect(),
        span_chirho: gen_span_chirho(),
    }
}

fn fixity_chirho(precedence_chirho: Option<u8>, op_chirho: &str) -> DeclChirho {
    DeclChirho::FixityDeclChirho {
        fixity_chirho: FixityChirho::InfixlChirho,
        precedence_chirho,
        ops_chirho: vec![var_name_chirho(op_chirho)],
        span_chirho: gen_span_chirho(),
    }
}

fn forms_chirho(metadata_chirho: &DerivedShowChirho) -> Vec<(&str, &ShowConFormChirho)> {
    metadata_chirho
        .constructors_chirho
        .iter()
        .map(|constructor_chirho| {
            (
                constructor_chirho.name_chirho.as_str(),
                &constructor_chirho.form_chirho,
            )
        })
        .collect()
}

#[test]
fn a_binary_symbolic_constructor_renders_infix_at_its_declared_or_default_precedence_chirho() {
    let int_chirho = || con_type_chirho("Int");
    let metadata_chirho = derived_show_metadata_chirho(&module_chirho(vec![
        fixity_chirho(Some(6), ":+:"),
        fixity_chirho(None, ":-:"),
        data_chirho(
            "EChirho",
            vec![
                ordinary_chirho(":+:", vec![int_chirho(), int_chirho()]),
                ordinary_chirho(":-:", vec![int_chirho(), int_chirho()]),
                ordinary_chirho(":*:", vec![int_chirho(), int_chirho()]),
            ],
            &["Show"],
        ),
    ]));
    // A declaration without a precedence, and no declaration at all, both mean infixl 9.
    assert_eq!(
        forms_chirho(&metadata_chirho["EChirho"]),
        vec![
            (":+:", &ShowConFormChirho::InfixChirho(6)),
            (":-:", &ShowConFormChirho::InfixChirho(9)),
            (":*:", &ShowConFormChirho::InfixChirho(9)),
        ]
    );
}

#[test]
fn only_a_binary_symbolic_constructor_renders_infix_chirho() {
    let int_chirho = || con_type_chirho("Int");
    let metadata_chirho = derived_show_metadata_chirho(&module_chirho(vec![data_chirho(
        "TChirho",
        vec![
            ordinary_chirho("PairChirho", vec![int_chirho(), int_chirho()]),
            ordinary_chirho(":!", vec![int_chirho()]),
            ordinary_chirho(":!!", vec![int_chirho(), int_chirho(), int_chirho()]),
            ordinary_chirho("NoneChirho", vec![]),
        ],
        &["Eq", "Show"],
    )]));
    assert!(
        forms_chirho(&metadata_chirho["TChirho"])
            .iter()
            .all(|(_, form_chirho)| **form_chirho == ShowConFormChirho::PrefixChirho)
    );
}

#[test]
fn record_labels_and_field_types_keep_declaration_order_chirho() {
    let field_chirho = |names_chirho: &[&str], ty_chirho: TypeChirho| FieldDeclChirho {
        names_chirho: names_chirho
            .iter()
            .map(|name_chirho| var_name_chirho(name_chirho))
            .collect(),
        ty_chirho,
        strictness_chirho: StrictnessChirho::LazyChirho,
        span_chirho: gen_span_chirho(),
    };
    let maybe_pair_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(con_type_chirho("Maybe")),
        arg_chirho: Box::new(TypeChirho::ParenChirho {
            inner_chirho: Box::new(TypeChirho::TupleChirho {
                elements_chirho: vec![
                    con_type_chirho("Int"),
                    TypeChirho::ListChirho {
                        element_chirho: Box::new(con_type_chirho("Char")),
                        span_chirho: gen_span_chirho(),
                    },
                ],
                span_chirho: gen_span_chirho(),
            }),
            span_chirho: gen_span_chirho(),
        }),
        span_chirho: gen_span_chirho(),
    };
    let metadata_chirho = derived_show_metadata_chirho(&module_chirho(vec![data_chirho(
        "RChirho",
        vec![ConDeclChirho::RecordChirho {
            name_chirho: var_name_chirho("RChirho"),
            fields_chirho: vec![
                field_chirho(&["xChirho", "yChirho"], con_type_chirho("Int")),
                field_chirho(&["zChirho"], maybe_pair_chirho),
            ],
            span_chirho: gen_span_chirho(),
        }],
        &["Show"],
    )]));
    let constructor_chirho = &metadata_chirho["RChirho"].constructors_chirho[0];
    assert_eq!(
        constructor_chirho.form_chirho,
        ShowConFormChirho::RecordChirho(vec![
            "xChirho".to_string(),
            "yChirho".to_string(),
            "zChirho".to_string()
        ])
    );
    let int_field_chirho = ShowFieldTypeChirho::ConChirho("Int".to_string());
    assert_eq!(
        constructor_chirho.fields_chirho,
        vec![
            int_field_chirho.clone(),
            int_field_chirho.clone(),
            ShowFieldTypeChirho::AppChirho(
                Box::new(ShowFieldTypeChirho::ConChirho("Maybe".to_string())),
                Box::new(ShowFieldTypeChirho::TupleChirho(vec![
                    int_field_chirho,
                    ShowFieldTypeChirho::ListChirho(Box::new(ShowFieldTypeChirho::ConChirho(
                        "Char".to_string()
                    ))),
                ])),
            ),
        ]
    );
}

#[test]
fn types_not_deriving_show_or_with_a_gadt_constructor_are_left_out_chirho() {
    let metadata_chirho = derived_show_metadata_chirho(&module_chirho(vec![
        data_chirho(
            "EqOnlyChirho",
            vec![ordinary_chirho("EqOnlyChirho", vec![])],
            &["Eq"],
        ),
        data_chirho(
            "GadtChirho",
            vec![ConDeclChirho::GadtChirho {
                name_chirho: var_name_chirho("MkGadtChirho"),
                ty_chirho: con_type_chirho("GadtChirho"),
                span_chirho: gen_span_chirho(),
            }],
            &["Show"],
        ),
        data_chirho(
            "KeptChirho",
            vec![ordinary_chirho("KeptChirho", vec![])],
            &["Show"],
        ),
    ]));
    let mut names_chirho: Vec<&String> = metadata_chirho.keys().collect();
    names_chirho.sort();
    assert_eq!(names_chirho, vec!["KeptChirho"]);
}
