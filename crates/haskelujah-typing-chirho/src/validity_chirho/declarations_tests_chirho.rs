// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The declaration rules on hand-built AST: each rule's negative, and the
//! neighbouring shapes it must leave alone.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{FieldDeclChirho, StrictnessChirho, TyVarChirho};
use haskelujah_ast_chirho::name_chirho::RawNameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho};

fn span_at_chirho(start_chirho: u32) -> SpanChirho {
    SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(start_chirho),
        ByteOffsetChirho::new_chirho(start_chirho + 1),
    )
}

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn con_ty_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(name_chirho(text_chirho))
}

fn var_ty_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

fn arrow_chirho(arg_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::FunChirho {
        arg_chirho: Box::new(arg_chirho),
        mult_chirho: None,
        result_chirho: Box::new(result_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `Show a => body`
fn show_context_chirho(body_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho("Show"),
            args_chirho: vec![var_ty_chirho("a")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(body_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `forall a. body`
fn forall_a_chirho(body_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("a"))],
        body_chirho: Box::new(body_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Test"),
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

fn newtype_chirho(constructor_chirho: ConDeclChirho) -> DeclChirho {
    DeclChirho::NewtypeDeclChirho {
        name_chirho: name_chirho("N"),
        type_vars_chirho: vec![],
        constructor_chirho,
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn ordinary_chirho(con_name_chirho: &str, field_count_chirho: usize) -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: name_chirho(con_name_chirho),
        fields_chirho: (0..field_count_chirho)
            .map(|_| (StrictnessChirho::LazyChirho, con_ty_chirho("Int")))
            .collect(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn record_chirho(field_names_chirho: &[&str]) -> ConDeclChirho {
    ConDeclChirho::RecordChirho {
        name_chirho: name_chirho("R"),
        fields_chirho: vec![FieldDeclChirho {
            multiplicity_chirho: None,
            names_chirho: field_names_chirho
                .iter()
                .copied()
                .map(name_chirho)
                .collect(),
            ty_chirho: con_ty_chirho("Int"),
            strictness_chirho: StrictnessChirho::LazyChirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn gadt_chirho(ty_chirho: TypeChirho) -> ConDeclChirho {
    ConDeclChirho::GadtChirho {
        name_chirho: name_chirho("MkN"),
        signature_chirho:
            haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho::from_type_chirho(ty_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn fun_bind_chirho(name_text_chirho: &str, start_chirho: u32) -> LocalBindChirho {
    LocalBindChirho::FunBindChirho {
        name_chirho: name_chirho(name_text_chirho),
        matches_chirho: vec![],
        span_chirho: span_at_chirho(start_chirho),
    }
}

fn instance_chirho(methods_chirho: Vec<LocalBindChirho>) -> DeclChirho {
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: name_chirho("Eq"),
        types_chirho: vec![con_ty_chirho("T")],
        methods_chirho,
        assoc_tf_instances_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn class_chirho(default_sig_chirho: Option<&str>, has_default_chirho: bool) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho("C"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("a"))],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: name_chirho("meth"),
            ty_chirho: var_ty_chirho("a"),
            default_chirho: has_default_chirho.then(Vec::new),
            default_sig_chirho: default_sig_chirho.map(str::to_string),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// The GHC codes reported for a module of these declarations, in order.
fn codes_chirho(decls_chirho: Vec<DeclChirho>) -> Vec<&'static str> {
    check_module_declaration_validity_chirho(&module_chirho(decls_chirho))
        .iter()
        .map(|error_chirho| {
            error_chirho
                .ghc_code_chirho
                .expect("this semantic declaration rule has a measured GHC code")
        })
        .collect()
}

#[test]
fn a_newtype_constructor_needs_exactly_one_field_chirho() {
    // T23308's `B1 C1 C1` and T15523's bare `Foo`.
    assert_eq!(
        codes_chirho(vec![newtype_chirho(ordinary_chirho("B1", 2))]),
        ["GHC-23517"]
    );
    assert_eq!(
        codes_chirho(vec![newtype_chirho(ordinary_chirho("Foo", 0))]),
        ["GHC-23517"]
    );
    assert!(codes_chirho(vec![newtype_chirho(ordinary_chirho("N", 1))]).is_empty());
}

#[test]
fn a_newtype_record_counts_its_field_names_chirho() {
    // `{ leftChirho, rightChirho :: Int }` is one declaration of two fields.
    assert_eq!(
        codes_chirho(vec![newtype_chirho(record_chirho(&["left", "right"]))]),
        ["GHC-23517"]
    );
    assert!(codes_chirho(vec![newtype_chirho(record_chirho(&["unR"]))]).is_empty());
}

#[test]
fn a_placeholder_constructor_is_not_judged_chirho() {
    // Lowering's empty name stands for a constructor it could not read
    // (`newtype N = (:|) Int`, which GHC accepts) or for none at all.
    assert!(codes_chirho(vec![newtype_chirho(ordinary_chirho("", 2))]).is_empty());
    assert!(codes_chirho(vec![newtype_chirho(ordinary_chirho("", 0))]).is_empty());
}

#[test]
fn a_gadt_newtype_constructor_context_is_reported_chirho() {
    // T15796: `MkN :: Show a => a -> N a`, with and without a leading forall.
    let field_chirho = arrow_chirho(var_ty_chirho("a"), con_ty_chirho("N"));
    assert_eq!(
        codes_chirho(vec![newtype_chirho(gadt_chirho(show_context_chirho(
            field_chirho.clone()
        )))]),
        ["GHC-17440"]
    );
    assert_eq!(
        codes_chirho(vec![newtype_chirho(gadt_chirho(forall_a_chirho(
            show_context_chirho(field_chirho.clone())
        )))]),
        ["GHC-17440"]
    );
    assert!(
        codes_chirho(vec![newtype_chirho(gadt_chirho(forall_a_chirho(
            field_chirho
        )))])
        .is_empty()
    );
}

#[test]
fn a_context_inside_a_gadt_field_is_not_the_constructor_context_chirho() {
    // `MkN :: (forall a. Show a => a -> String) -> N` has a rank-N field.
    let rank_n_chirho = TypeChirho::ParenChirho {
        inner_chirho: Box::new(forall_a_chirho(show_context_chirho(arrow_chirho(
            var_ty_chirho("a"),
            con_ty_chirho("String"),
        )))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        codes_chirho(vec![newtype_chirho(gadt_chirho(arrow_chirho(
            rank_n_chirho,
            con_ty_chirho("N")
        )))])
        .is_empty()
    );
}

#[test]
fn data_declarations_are_not_judged_as_newtypes_chirho() {
    let data_chirho = DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("D"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![
            ordinary_chirho("Pair", 2),
            gadt_chirho(show_context_chirho(arrow_chirho(
                var_ty_chirho("a"),
                con_ty_chirho("D"),
            ))),
        ],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(codes_chirho(vec![data_chirho]).is_empty());
}

#[test]
fn a_name_bound_apart_in_one_instance_is_reported_at_each_repeat_chirho() {
    // tcfail038: `==`, `/=`, `==`, `/=`.
    let errors_chirho =
        check_module_declaration_validity_chirho(&module_chirho(vec![instance_chirho(vec![
            fun_bind_chirho("==", 10),
            fun_bind_chirho("/=", 20),
            fun_bind_chirho("==", 30),
            fun_bind_chirho("/=", 40),
        ])]));
    let found_chirho: Vec<_> = errors_chirho
        .iter()
        .map(|error_chirho| {
            (
                error_chirho
                    .ghc_code_chirho
                    .expect("this semantic declaration rule has a measured GHC code"),
                error_chirho.span_chirho,
                error_chirho.related_span_chirho,
            )
        })
        .collect();
    assert_eq!(
        found_chirho,
        [
            ("GHC-10498", span_at_chirho(30), Some(span_at_chirho(10))),
            ("GHC-10498", span_at_chirho(40), Some(span_at_chirho(20))),
        ]
    );
}

#[test]
fn signatures_distinct_names_and_other_instances_are_not_conflicts_chirho() {
    let signature_chirho = LocalBindChirho::TypeSigChirho {
        name_chirho: name_chirho("show"),
        ty_chirho: arrow_chirho(con_ty_chirho("T"), con_ty_chirho("String")),
        span_chirho: span_at_chirho(5),
    };
    assert!(
        codes_chirho(vec![instance_chirho(vec![
            signature_chirho,
            fun_bind_chirho("show", 10),
            fun_bind_chirho("showsPrec", 20),
        ])])
        .is_empty()
    );
    assert!(
        codes_chirho(vec![
            instance_chirho(vec![fun_bind_chirho("==", 10)]),
            instance_chirho(vec![fun_bind_chirho("==", 20)]),
        ])
        .is_empty()
    );
}

#[test]
fn a_default_signature_needs_a_default_binding_chirho() {
    // MissingDefaultMethodBinding: `default meth :: Num a => a` and no `meth = ...`.
    assert_eq!(
        codes_chirho(vec![class_chirho(Some("Num a => a"), false)]),
        ["GHC-59794"]
    );
    assert!(codes_chirho(vec![class_chirho(Some("Num a => a"), true)]).is_empty());
    assert!(codes_chirho(vec![class_chirho(None, true)]).is_empty());
    assert!(codes_chirho(vec![class_chirho(None, false)]).is_empty());
}

#[test]
fn findings_become_e0209_diagnostics_naming_the_ghc_code_chirho() {
    let result_chirho =
        check_module_declaration_validity_diagnostics_chirho(&module_chirho(vec![newtype_chirho(
            ordinary_chirho("B1", 2),
        )]));
    assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
    let rendered_chirho = result_chirho.diagnostics_chirho.to_string();
    assert!(rendered_chirho.contains("E0209"), "{rendered_chirho}");
    assert!(rendered_chirho.contains("GHC-23517"), "{rendered_chirho}");
}

/// One positional constructor whose `Int` fields carry these annotations.
fn annotated_chirho(
    con_name_chirho: &str,
    strictness_chirho: &[StrictnessChirho],
) -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: name_chirho(con_name_chirho),
        fields_chirho: strictness_chirho
            .iter()
            .map(|&annotation_chirho| (annotation_chirho, con_ty_chirho("Int")))
            .collect(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// A one-field record constructor whose field carries this annotation.
fn annotated_record_chirho(strictness_chirho: StrictnessChirho) -> ConDeclChirho {
    ConDeclChirho::RecordChirho {
        name_chirho: name_chirho("R"),
        fields_chirho: vec![FieldDeclChirho {
            names_chirho: vec![name_chirho("fieldChirho")],
            ty_chirho: con_ty_chirho("Int"),
            strictness_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn data_decl_chirho(constructors_chirho: Vec<ConDeclChirho>) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("D"),
        type_vars_chirho: vec![],
        constructors_chirho,
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// The GHC codes for these declarations under these LANGUAGE extensions, in order.
fn codes_with_chirho(
    extensions_chirho: &[&str],
    decls_chirho: Vec<DeclChirho>,
) -> Vec<&'static str> {
    let mut test_module_chirho = module_chirho(decls_chirho);
    test_module_chirho.extensions_chirho = extensions_chirho
        .iter()
        .map(|extension_chirho| extension_chirho.to_string())
        .collect();
    check_module_declaration_validity_chirho(&test_module_chirho)
        .iter()
        .map(|error_chirho| {
            error_chirho
                .ghc_code_chirho
                .expect("strictness controls produce semantic GHC diagnostics")
        })
        .collect()
}

#[test]
fn a_newtype_field_takes_no_strictness_annotation_chirho() {
    // L04/L08 (`~`, with and without StrictData) and readFail008 (`!`): GHC-04049,
    // never GHC-81601 as well.
    for extensions_chirho in [&[][..], &["StrictData"][..]] {
        for annotation_chirho in [
            StrictnessChirho::ExplicitLazyChirho,
            StrictnessChirho::StrictChirho,
            StrictnessChirho::UnpackChirho,
        ] {
            for constructor_chirho in [
                annotated_chirho("N", &[annotation_chirho]),
                annotated_record_chirho(annotation_chirho),
            ] {
                assert_eq!(
                    codes_with_chirho(extensions_chirho, vec![newtype_chirho(constructor_chirho)]),
                    ["GHC-04049"],
                    "{annotation_chirho:?} under {extensions_chirho:?}"
                );
            }
        }
        assert!(
            codes_with_chirho(
                extensions_chirho,
                vec![newtype_chirho(annotated_chirho(
                    "N",
                    &[StrictnessChirho::LazyChirho]
                ))]
            )
            .is_empty()
        );
    }
    // Rule 1 comes first, as in GHC: two annotated fields are one GHC-23517.
    assert_eq!(
        codes_with_chirho(
            &[],
            vec![newtype_chirho(annotated_chirho(
                "N",
                &[
                    StrictnessChirho::StrictChirho,
                    StrictnessChirho::StrictChirho
                ]
            ))]
        ),
        ["GHC-23517"]
    );
}

#[test]
fn a_lazy_annotation_needs_strict_data_chirho() {
    // L05/L06: `~` without StrictData is GHC-81601 at each constructor; Strict
    // implies StrictData, and the last mention wins.
    let lazy_decls_chirho = || {
        vec![data_decl_chirho(vec![
            annotated_chirho(
                "Pair",
                &[
                    StrictnessChirho::ExplicitLazyChirho,
                    StrictnessChirho::LazyChirho,
                ],
            ),
            annotated_record_chirho(StrictnessChirho::ExplicitLazyChirho),
        ])]
    };
    assert_eq!(
        codes_with_chirho(&[], lazy_decls_chirho()),
        ["GHC-81601", "GHC-81601"]
    );
    assert!(codes_with_chirho(&["StrictData"], lazy_decls_chirho()).is_empty());
    assert!(codes_with_chirho(&["Strict"], lazy_decls_chirho()).is_empty());
    assert_eq!(
        codes_with_chirho(&["StrictData", "NoStrictData"], lazy_decls_chirho()),
        ["GHC-81601", "GHC-81601"]
    );
}

#[test]
fn strict_and_unannotated_data_fields_need_no_extension_chirho() {
    // `!`, UNPACK and the default are legal in any module; lowering's placeholder
    // constructor is not judged.
    assert!(
        codes_with_chirho(
            &[],
            vec![data_decl_chirho(vec![annotated_chirho(
                "Pair",
                &[
                    StrictnessChirho::StrictChirho,
                    StrictnessChirho::UnpackChirho,
                    StrictnessChirho::LazyChirho,
                ],
            )])]
        )
        .is_empty()
    );
    assert!(
        codes_with_chirho(
            &[],
            vec![data_decl_chirho(vec![annotated_chirho(
                "",
                &[StrictnessChirho::ExplicitLazyChirho],
            )])]
        )
        .is_empty()
    );
}
