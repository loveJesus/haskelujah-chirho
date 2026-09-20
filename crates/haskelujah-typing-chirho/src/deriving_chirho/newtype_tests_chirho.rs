// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::newtype_chirho::eta_reduce_chirho;
use super::{NameChirho, RawNameChirho, SpanChirho, TyVarChirho, TypeChirho};
use haskelujah_ast_chirho::decl_chirho::AstKindChirho;

fn variable_chirho(name_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        name_chirho,
        SpanChirho::DUMMY_CHIRHO,
    )))
}

#[test]
fn eta_reduction_bounds_deep_and_wide_syntax_chirho() {
    let mut deep_chirho = variable_chirho("aChirho");
    for _depth_chirho in 0..140 {
        deep_chirho = TypeChirho::ParenChirho {
            inner_chirho: Box::new(deep_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    }
    let wide_chirho = TypeChirho::TupleChirho {
        elements_chirho: vec![variable_chirho("aChirho"); 16_385],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    for syntax_chirho in [&deep_chirho, &wide_chirho] {
        assert!(
            eta_reduce_chirho(syntax_chirho, &[])
                .unwrap_err()
                .contains("work limit")
        );
    }
}

#[test]
fn eta_reduction_checks_binder_classifiers_before_shadowing_chirho() {
    let TypeChirho::VarChirho(name_chirho) = variable_chirho("aChirho") else {
        unreachable!()
    };
    let removed_chirho = TyVarChirho::plain_chirho(name_chirho.clone());
    let prefix_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho::annotated_chirho(
            name_chirho,
            AstKindChirho::VarChirho("aChirho".to_owned()),
        )],
        body_chirho: Box::new(variable_chirho("aChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let syntax_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(prefix_chirho),
        arg_chirho: Box::new(variable_chirho("aChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        eta_reduce_chirho(&syntax_chirho, &[removed_chirho])
            .unwrap_err()
            .contains("representation prefix")
    );
}
