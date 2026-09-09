// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};

use super::signature_binders_chirho::ordered_scheme_vars_chirho;
use super::{InferCtxChirho, TyVarChirho};

fn name_chirho(text_chirho: &str, offset_chirho: u32) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::new_chirho(
            FileIdChirho::SYNTHETIC_CHIRHO,
            ByteOffsetChirho::new_chirho(offset_chirho),
            ByteOffsetChirho::new_chirho(offset_chirho + 1),
        ),
    ))
}

fn var_chirho(text_chirho: &str, offset_chirho: u32) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho, offset_chirho))
}

fn pair_chirho(left_chirho: TypeChirho, right_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::TupleChirho {
        elements_chirho: vec![left_chirho, right_chirho],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn assert_scheme_order_chirho(signature_chirho: &TypeChirho, names_chirho: &[&str]) {
    let (scheme_chirho, map_chirho) =
        InferCtxChirho::new_chirho().ast_type_to_scheme_with_var_map_chirho(signature_chirho);
    let expected_chirho: Vec<_> = names_chirho
        .iter()
        .map(|name_chirho| map_chirho[*name_chirho])
        .collect();
    assert_eq!(scheme_chirho.vars_chirho, expected_chirho);
}

#[test]
fn normalized_infix_spine_retains_lexical_quantifier_order_chirho() {
    let signature_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(var_chirho("opChirho", 20)),
            arg_chirho: Box::new(var_chirho("aChirho", 10)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(var_chirho("bChirho", 30)),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_scheme_order_chirho(&signature_chirho, &["aChirho", "opChirho", "bChirho"]);
}

#[test]
fn constraint_first_occurrences_precede_body_allocation_chirho() {
    let signature_chirho = TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho("Eq", 5),
            args_chirho: vec![var_chirho("bChirho", 10)],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(pair_chirho(
            var_chirho("aChirho", 20),
            var_chirho("bChirho", 30),
        )),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_scheme_order_chirho(&signature_chirho, &["bChirho", "aChirho"]);
}

#[test]
fn explicit_forall_order_is_not_replaced_by_occurrence_order_chirho() {
    let signature_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![
            AstTyVarChirho::plain_chirho(name_chirho("bChirho", 5)),
            AstTyVarChirho::plain_chirho(name_chirho("aChirho", 10)),
        ],
        body_chirho: Box::new(pair_chirho(
            var_chirho("aChirho", 20),
            var_chirho("bChirho", 30),
        )),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_scheme_order_chirho(&signature_chirho, &["bChirho", "aChirho"]);
}

#[test]
fn locally_bound_occurrences_do_not_order_an_outer_namesake_chirho() {
    // This isolates the inventory's lexical scope contract; it does not assert
    // that the separate AST-to-type converter supports all binder shadowing.
    let local_chirho = TypeChirho::RequiredForallChirho {
        vars_chirho: vec![AstTyVarChirho::plain_chirho(name_chirho("aChirho", 1))],
        body_chirho: Box::new(TypeChirho::ForallChirho {
            vars_chirho: vec![AstTyVarChirho::plain_chirho(name_chirho("aChirho", 2))],
            body_chirho: Box::new(var_chirho("aChirho", 3)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let signature_chirho = pair_chirho(
        local_chirho,
        pair_chirho(var_chirho("bChirho", 10), var_chirho("aChirho", 20)),
    );
    let map_chirho = HashMap::from([
        ("aChirho".to_string(), TyVarChirho(0)),
        ("bChirho".to_string(), TyVarChirho(1)),
    ]);
    assert_eq!(
        ordered_scheme_vars_chirho(
            &signature_chirho,
            &map_chirho,
            Vec::new(),
            &[TyVarChirho(0), TyVarChirho(1)]
        ),
        vec![TyVarChirho(1), TyVarChirho(0)]
    );
}

#[test]
fn generated_variables_have_deterministic_fallback_after_source_binders_chirho() {
    let map_chirho = HashMap::from([
        ("generatedFirstChirho".to_string(), TyVarChirho(8)),
        ("sourceChirho".to_string(), TyVarChirho(9)),
        ("generatedSecondChirho".to_string(), TyVarChirho(7)),
    ]);
    assert_eq!(
        ordered_scheme_vars_chirho(
            &var_chirho("sourceChirho", 10),
            &map_chirho,
            vec![TyVarChirho(6)],
            &[TyVarChirho(7), TyVarChirho(8), TyVarChirho(9)]
        ),
        vec![
            TyVarChirho(6),
            TyVarChirho(9),
            TyVarChirho(7),
            TyVarChirho(8)
        ]
    );
}
