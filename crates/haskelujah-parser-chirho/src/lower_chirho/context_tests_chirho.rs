// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Context retention is an AST contract: checker evidence cannot recover dropped binders.
use super::*;

fn class_context_chirho(source_chirho: &str) -> Vec<ConstraintChirho> {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::ClassDeclChirho { context_chirho, .. } = &module_chirho.decls_chirho[0] else {
        panic!("expected the class declaration");
    };
    context_chirho.clone()
}

#[test]
fn quantified_superclass_scopes_over_its_premise_and_conclusion_chirho() {
    let context_chirho = class_context_chirho(
        "module MChirho where\nclass (forall (aChirho :: Type). Eq aChirho => CChirho fChirho aChirho) => DChirho fChirho\n",
    );
    assert_eq!(context_chirho.len(), 1);
    let ConstraintChirho::QuantifiedChirho {
        vars_chirho,
        context_chirho,
        body_chirho,
        ..
    } = &context_chirho[0]
    else {
        panic!("forall must enclose both the premise and the conclusion");
    };
    assert_eq!(vars_chirho.len(), 1);
    assert_eq!(vars_chirho[0].text_chirho(), "aChirho");
    assert_eq!(
        vars_chirho[0].kind_annotation_chirho,
        Some(AstKindChirho::StarChirho)
    );
    assert_eq!(context_chirho.len(), 1);
    assert_eq!(
        context_chirho[0]
            .simple_class_chirho()
            .unwrap()
            .text_chirho(),
        "Eq"
    );
    assert_eq!(
        body_chirho.simple_class_chirho().unwrap().text_chirho(),
        "CChirho"
    );
    assert_eq!(body_chirho.simple_args_chirho().unwrap().len(), 2);
}

#[test]
fn variable_predicate_heads_are_retained_not_fabricated_chirho() {
    let context_chirho = class_context_chirho(
        "module MChirho where\nclass (forall aChirho. fChirho aChirho) => LimitChirho fChirho\n",
    );
    let ConstraintChirho::QuantifiedChirho { body_chirho, .. } = &context_chirho[0] else {
        panic!("quantified superclass disappeared");
    };
    assert_eq!(
        body_chirho.simple_class_chirho().unwrap().text_chirho(),
        "fChirho"
    );
    assert_eq!(body_chirho.simple_args_chirho().unwrap().len(), 1);
}

#[test]
fn superclass_tuple_keeps_each_application_intact_chirho() {
    let context_chirho = class_context_chirho(
        "module MChirho where\nclass (CChirho (fChirho aChirho), DChirho bChirho) => EChirho fChirho aChirho bChirho\n",
    );
    assert_eq!(context_chirho.len(), 2);
    assert_eq!(
        context_chirho[0]
            .simple_class_chirho()
            .unwrap()
            .text_chirho(),
        "CChirho"
    );
    assert_eq!(context_chirho[0].simple_args_chirho().unwrap().len(), 1);
    let TypeChirho::ParenChirho { inner_chirho, .. } =
        &context_chirho[0].simple_args_chirho().unwrap()[0]
    else {
        panic!("parenthesized superclass argument must survive");
    };
    assert!(matches!(
        inner_chirho.as_ref(),
        TypeChirho::AppChirho { .. }
    ));
    assert_eq!(
        context_chirho[1]
            .simple_class_chirho()
            .unwrap()
            .text_chirho(),
        "DChirho"
    );
}
