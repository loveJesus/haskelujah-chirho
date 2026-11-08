// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-type boundaries measured with GHC 9.14.1 in row 686's
//! tuple-arity-boundary-chirho oracle: 63/64 accept, 65 rejects GHC-94803.

use super::super::check_module_type_validity_chirho;
use super::*;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, StrictnessChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn int_chirho() -> TypeChirho {
    TypeChirho::ConChirho(name_chirho("Int"))
}

fn tuple_chirho(arity_chirho: usize) -> TypeChirho {
    TypeChirho::TupleChirho {
        elements_chirho: (0..arity_chirho).map(|_| int_chirho()).collect(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn constructor_name_chirho(arity_chirho: usize) -> String {
    format!("({})", ",".repeat(arity_chirho.saturating_sub(1)))
}

fn application_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn prefix_tuple_chirho(arity_chirho: usize, promoted_chirho: bool) -> TypeChirho {
    let name_chirho = name_chirho(&constructor_name_chirho(arity_chirho));
    let head_chirho = if promoted_chirho {
        TypeChirho::PromotedConChirho {
            name_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    } else {
        TypeChirho::ConChirho(name_chirho)
    };
    (0..arity_chirho).fold(head_chirho, |fun_chirho, _| {
        application_chirho(fun_chirho, int_chirho())
    })
}

fn module_chirho(decl_chirho: DeclChirho) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("TupleBoundaryChirho"),
        exports_chirho: None,
        imports_chirho: Vec::new(),
        decls_chirho: vec![decl_chirho],
        extensions_chirho: vec![
            "DataKinds".to_owned(),
            "ConstraintKinds".to_owned(),
            "QuantifiedConstraints".to_owned(),
        ],
        inline_pragmas_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: Vec::new(),
        deriving_via_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn signature_module_chirho(ty_chirho: TypeChirho) -> ModuleChirho {
    module_chirho(DeclChirho::TypeSigChirho {
        name_chirho: name_chirho("valueChirho"),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    })
}

fn assert_boundary_chirho(module_chirho: &ModuleChirho, arity_chirho: usize) {
    let errors_chirho = check_module_type_validity_chirho(module_chirho);
    assert_eq!(
        errors_chirho.len(),
        usize::from(arity_chirho > 64),
        "arity {arity_chirho}: {errors_chirho:?}"
    );
    for error_chirho in errors_chirho {
        assert!(error_chirho.message_chirho.contains("GHC-94803"));
        assert!(
            error_chirho
                .message_chirho
                .contains(&format!("{arity_chirho}-tuple"))
        );
        assert!(error_chirho.suggested_extension_chirho.is_none());
        assert_eq!(error_chirho.span_chirho, SpanChirho::DUMMY_CHIRHO);
    }
}

#[test]
fn ordinary_tuple_types_keep_63_and_64_but_reject_65_chirho() {
    // Boxed and unboxed source tuple types share this AST node; the fresh oracle
    // establishes the same boundary for both, so no boxity inference is needed.
    for arity_chirho in [63, 64, 65] {
        assert_boundary_chirho(
            &signature_module_chirho(tuple_chirho(arity_chirho)),
            arity_chirho,
        );
    }
}

#[test]
fn ordinary_and_promoted_prefix_constructors_keep_their_own_arity_chirho() {
    for arity_chirho in [63, 64, 65] {
        for promoted_chirho in [false, true] {
            assert_boundary_chirho(
                &signature_module_chirho(prefix_tuple_chirho(arity_chirho, promoted_chirho)),
                arity_chirho,
            );
        }
    }
    // Unsaturated prefix tuple constructors are already oversized: their own
    // comma count is authoritative, rather than the application argument count.
    assert_boundary_chirho(
        &signature_module_chirho(TypeChirho::PromotedConChirho {
            name_chirho: name_chirho(&constructor_name_chirho(65)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        65,
    );
}

#[test]
fn t18723b_promoted_tuple_in_a_constructor_field_is_refused_chirho() {
    // This is T18723b's preserved `MkT2 (Proxy '(Int,...65))` type shape.
    let field_chirho = application_chirho(
        TypeChirho::ConChirho(name_chirho("Proxy")),
        prefix_tuple_chirho(65, true),
    );
    let module_chirho = module_chirho(DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("T2"),
        type_vars_chirho: Vec::new(),
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            prefix_chirho: Default::default(),
            name_chirho: name_chirho("MkT2"),
            fields_chirho: vec![(StrictnessChirho::LazyChirho, field_chirho)],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: Vec::new(),
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
    assert_boundary_chirho(&module_chirho, 65);
}

#[test]
fn constraint_tuple_synonym_rhs_uses_the_same_size_boundary_chirho() {
    for arity_chirho in [63, 64, 65] {
        let rhs_chirho = TypeChirho::TupleChirho {
            elements_chirho: (0..arity_chirho)
                .map(|_| application_chirho(TypeChirho::ConChirho(name_chirho("Eq")), int_chirho()))
                .collect(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert_boundary_chirho(
            &module_chirho(DeclChirho::TypeAliasDeclChirho {
                name_chirho: name_chirho("ContextChirho"),
                type_vars_chirho: Vec::new(),
                rhs_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arity_chirho,
        );
    }
}

fn constraint_chirho(class_chirho: &str, args_chirho: Vec<TypeChirho>) -> ConstraintChirho {
    ConstraintChirho::ClassChirho {
        kind_args_chirho: Vec::new(),
        class_chirho: Box::new(name_chirho(class_chirho)),
        args_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn qualified_type_chirho(context_chirho: Vec<ConstraintChirho>) -> TypeChirho {
    TypeChirho::QualChirho {
        context_chirho,
        body_chirho: Box::new(int_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn tuple_constructor_constraint_heads_cannot_bypass_the_type_gateway_chirho() {
    for arity_chirho in [63, 64, 65] {
        assert_boundary_chirho(
            &signature_module_chirho(qualified_type_chirho(vec![constraint_chirho(
                &constructor_name_chirho(arity_chirho),
                (0..arity_chirho).map(|_| int_chirho()).collect(),
            )])),
            arity_chirho,
        );
    }
}

#[test]
fn quantified_constraint_types_and_context_heads_are_checked_chirho() {
    for context_chirho in [false, true] {
        let oversized_chirho = if context_chirho {
            constraint_chirho(&constructor_name_chirho(65), Vec::new())
        } else {
            constraint_chirho("Show", vec![prefix_tuple_chirho(65, true)])
        };
        let quantified_chirho = ConstraintChirho::QuantifiedChirho {
            vars_chirho: Vec::new(),
            context_chirho: if context_chirho {
                vec![oversized_chirho.clone()]
            } else {
                Vec::new()
            },
            body_chirho: Box::new(if context_chirho {
                constraint_chirho("Eq", vec![int_chirho()])
            } else {
                oversized_chirho
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert_boundary_chirho(
            &signature_module_chirho(qualified_type_chirho(vec![quantified_chirho])),
            65,
        );
    }
}

#[test]
fn flattened_constraint_count_does_not_invent_an_oversized_tuple_chirho() {
    // `(Eq a, (64 legal inner constraints))` flattens into this vector.
    // Its written tuple arities are 2 and 64, not 65.
    let context_chirho = (0..65)
        .map(|_| constraint_chirho("Eq", vec![int_chirho()]))
        .collect();
    assert!(
        check_module_type_validity_chirho(&signature_module_chirho(qualified_type_chirho(
            context_chirho
        )))
        .is_empty()
    );
}

#[test]
fn promoted_lists_and_non_tuple_symbols_have_no_tuple_size_cap_chirho() {
    let list_chirho = TypeChirho::PromotedListChirho {
        elements_chirho: (0..65).map(|_| int_chirho()).collect(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(check_module_type_validity_chirho(&signature_module_chirho(list_chirho)).is_empty());
    let mut errors_chirho = Vec::new();
    check_type_tuple_arity_chirho(
        &TypeChirho::ConChirho(name_chirho("(+:)")),
        &mut errors_chirho,
    );
    assert!(errors_chirho.is_empty());
}
