// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{InferCtxChirho, TyChirho};

#[test]
fn synonym_substitution_is_simultaneous_and_preserves_linear_arrows_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.register_type_synonym_chirho(
        "LinearPairChirho".into(),
        vec!["leftChirho".into(), "rightChirho".into()],
        TyChirho::FunChirho(
            Box::new(TyChirho::ForallVarChirho("leftChirho".into())),
            Box::new(TyChirho::ForallVarChirho("rightChirho".into())),
            super::MultChirho::OneChirho,
        ),
    );
    let caller_chirho = TyChirho::ForallVarChirho("rightChirho".into());
    let application_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("LinearPairChirho".into())),
            Box::new(caller_chirho.clone()),
        )),
        Box::new(TyChirho::int_chirho()),
    );
    assert_eq!(
        context_chirho.expand_type_synonyms_chirho(&application_chirho),
        TyChirho::FunChirho(
            Box::new(caller_chirho),
            Box::new(TyChirho::int_chirho()),
            super::MultChirho::OneChirho,
        ),
    );
}

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

#[test]
fn data_head_keeps_invisible_binders_lexical_and_applies_visible_parameters_chirho() {
    use haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho;
    let mut invisible_chirho = AstTyVarChirho::plain_chirho(name_chirho("jChirho"));
    invisible_chirho.visibility_chirho = TyVarVisibilityChirho::InvisibleChirho;
    let visible_chirho = AstTyVarChirho::plain_chirho(name_chirho("aChirho"));
    let mut context_chirho = InferCtxChirho::new_chirho();
    let (result_chirho, mut map_chirho) =
        context_chirho.data_head_type_chirho("BoxChirho", &[invisible_chirho, visible_chirho]);
    assert_eq!(map_chirho.len(), 2);
    assert_eq!(
        result_chirho,
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("BoxChirho".into())),
            Box::new(TyChirho::VarChirho(map_chirho["aChirho"]))
        )
    );
    let invisible_id_chirho = map_chirho["jChirho"];
    assert_eq!(
        context_chirho.ast_type_to_ty_chirho(&var_chirho("jChirho"), &mut map_chirho),
        TyChirho::VarChirho(invisible_id_chirho)
    );
}

fn tuple_chirho(elements_chirho: Vec<TypeChirho>) -> TypeChirho {
    TypeChirho::TupleChirho {
        elements_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn forall_chirho(required_chirho: bool, body_chirho: TypeChirho) -> TypeChirho {
    let vars_chirho = vec![AstTyVarChirho::plain_chirho(name_chirho("aChirho"))];
    if required_chirho {
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho: Box::new(body_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    } else {
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho: Box::new(body_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }
}

fn quantified_body_chirho(ty_chirho: &TyChirho, required_chirho: bool) -> &TyChirho {
    match ty_chirho {
        TyChirho::ForallChirho { body_chirho, .. } if !required_chirho => body_chirho,
        TyChirho::RequiredForallChirho { body_chirho, .. } if required_chirho => body_chirho,
        _ => panic!("quantifier visibility changed: {ty_chirho:?}"),
    }
}

#[test]
fn forall_shadow_restores_the_enclosing_type_identity_chirho() {
    for required_chirho in [false, true] {
        let mut context_chirho = InferCtxChirho::new_chirho();
        let mut map_chirho = HashMap::new();
        let outer_chirho =
            context_chirho.ast_type_to_ty_chirho(&var_chirho("aChirho"), &mut map_chirho);
        let signature_chirho = tuple_chirho(vec![
            var_chirho("aChirho"),
            forall_chirho(required_chirho, var_chirho("aChirho")),
            var_chirho("aChirho"),
        ]);
        let TyChirho::TupleChirho(elements_chirho) =
            context_chirho.ast_type_to_ty_chirho(&signature_chirho, &mut map_chirho)
        else {
            panic!("expected the three type positions");
        };
        assert_eq!(elements_chirho[0], outer_chirho);
        assert_eq!(elements_chirho[2], outer_chirho);
        assert_ne!(
            quantified_body_chirho(&elements_chirho[1], required_chirho),
            &outer_chirho
        );
        assert_eq!(TyChirho::VarChirho(map_chirho["aChirho"]), outer_chirho);
    }
}

#[test]
fn forall_local_names_do_not_escape_but_new_free_names_survive_chirho() {
    for required_chirho in [false, true] {
        let mut context_chirho = InferCtxChirho::new_chirho();
        let mut map_chirho = HashMap::new();
        let signature_chirho = forall_chirho(
            required_chirho,
            tuple_chirho(vec![var_chirho("aChirho"), var_chirho("freeChirho")]),
        );
        let converted_chirho =
            context_chirho.ast_type_to_ty_chirho(&signature_chirho, &mut map_chirho);
        assert!(
            !map_chirho.contains_key("aChirho"),
            "local binder escaped its scope"
        );
        let free_chirho = TyChirho::VarChirho(map_chirho["freeChirho"]);
        let TyChirho::TupleChirho(elements_chirho) =
            quantified_body_chirho(&converted_chirho, required_chirho)
        else {
            panic!("expected the bound and free type positions");
        };
        assert_eq!(elements_chirho[1], free_chirho);
        assert_eq!(
            context_chirho.ast_type_to_ty_chirho(&var_chirho("freeChirho"), &mut map_chirho),
            free_chirho
        );
        assert_ne!(
            context_chirho.ast_type_to_ty_chirho(&var_chirho("aChirho"), &mut map_chirho),
            elements_chirho[0]
        );
    }
}

#[test]
fn nested_forall_shadows_restore_each_lexical_scope_chirho() {
    for required_chirho in [false, true] {
        let mut context_chirho = InferCtxChirho::new_chirho();
        let mut map_chirho = HashMap::new();
        let signature_chirho = forall_chirho(
            required_chirho,
            tuple_chirho(vec![
                var_chirho("aChirho"),
                forall_chirho(!required_chirho, var_chirho("aChirho")),
                var_chirho("aChirho"),
            ]),
        );
        let converted_chirho =
            context_chirho.ast_type_to_ty_chirho(&signature_chirho, &mut map_chirho);
        let TyChirho::TupleChirho(elements_chirho) =
            quantified_body_chirho(&converted_chirho, required_chirho)
        else {
            panic!("expected nested type scopes");
        };
        assert_eq!(elements_chirho[0], elements_chirho[2]);
        assert_ne!(
            &elements_chirho[0],
            quantified_body_chirho(&elements_chirho[1], !required_chirho)
        );
        assert!(
            map_chirho.is_empty(),
            "only locally quantified variables occur"
        );
    }
}

fn eq_context_chirho(body_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho("Eq"),
            args_chirho: vec![var_chirho("aChirho")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(body_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn result_predicates_reuse_their_lexically_quantified_type_identity_chirho() {
    for required_chirho in [false, true] {
        let signature_chirho = forall_chirho(
            false,
            eq_context_chirho(TypeChirho::FunChirho {
                arg_chirho: Box::new(var_chirho("aChirho")),
                mult_chirho: None,
                result_chirho: Box::new(forall_chirho(
                    required_chirho,
                    eq_context_chirho(var_chirho("aChirho")),
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
        );
        let (scheme_chirho, map_chirho) =
            InferCtxChirho::new_chirho().ast_type_to_scheme_with_var_map_chirho(&signature_chirho);
        let TyChirho::FunChirho(outer_chirho, result_chirho, _) = &scheme_chirho.ty_chirho else {
            panic!("expected outer argument and quantified result");
        };
        let inner_chirho = quantified_body_chirho(result_chirho, required_chirho);
        assert_eq!(scheme_chirho.preds_chirho.len(), 2);
        assert_eq!(
            &scheme_chirho.preds_chirho[0].ty_chirho,
            outer_chirho.as_ref()
        );
        assert_eq!(&scheme_chirho.preds_chirho[1].ty_chirho, inner_chirho);
        assert_ne!(outer_chirho.as_ref(), inner_chirho);
        assert_eq!(TyChirho::VarChirho(map_chirho["aChirho"]), **outer_chirho);
    }
}

#[test]
fn leading_predicates_precede_later_shadowing_and_keep_distinct_binders_chirho() {
    let signature_chirho = forall_chirho(
        false,
        eq_context_chirho(forall_chirho(
            false,
            eq_context_chirho(var_chirho("aChirho")),
        )),
    );
    let (scheme_chirho, map_chirho) =
        InferCtxChirho::new_chirho().ast_type_to_scheme_with_var_map_chirho(&signature_chirho);
    assert_eq!(scheme_chirho.vars_chirho.len(), 2);
    assert_eq!(scheme_chirho.preds_chirho.len(), 2);
    let outer_chirho = TyChirho::VarChirho(scheme_chirho.vars_chirho[0]);
    let inner_chirho = TyChirho::VarChirho(scheme_chirho.vars_chirho[1]);
    assert_ne!(outer_chirho, inner_chirho);
    assert_eq!(scheme_chirho.preds_chirho[0].ty_chirho, outer_chirho);
    assert_eq!(scheme_chirho.preds_chirho[1].ty_chirho, inner_chirho);
    assert_eq!(scheme_chirho.ty_chirho, inner_chirho);
    assert_eq!(TyChirho::VarChirho(map_chirho["aChirho"]), outer_chirho);
}

#[test]
fn parenthesized_signature_quantifies_each_written_binder_only_once_chirho() {
    let signature_chirho = TypeChirho::ParenChirho {
        inner_chirho: Box::new(forall_chirho(false, var_chirho("aChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (scheme_chirho, map_chirho) =
        InferCtxChirho::new_chirho().ast_type_to_scheme_with_var_map_chirho(&signature_chirho);
    assert_eq!(scheme_chirho.vars_chirho, vec![map_chirho["aChirho"]]);
    assert_eq!(
        scheme_chirho.ty_chirho,
        TyChirho::VarChirho(map_chirho["aChirho"])
    );
}

#[test]
fn explicit_local_signature_binder_shadows_an_enclosing_signature_binder_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let mut outer_map_chirho = HashMap::new();
    let outer_chirho =
        context_chirho.ast_type_to_ty_chirho(&var_chirho("aChirho"), &mut outer_map_chirho);
    context_chirho.scoped_tyvars_chirho = outer_map_chirho;
    let (scheme_chirho, map_chirho) = context_chirho
        .ast_type_to_scheme_with_var_map_chirho(&forall_chirho(false, var_chirho("aChirho")));
    assert_eq!(scheme_chirho.vars_chirho.len(), 1);
    assert_eq!(
        scheme_chirho.ty_chirho,
        TyChirho::VarChirho(map_chirho["aChirho"])
    );
    assert_ne!(scheme_chirho.ty_chirho, outer_chirho);
    assert_eq!(
        TyChirho::VarChirho(context_chirho.scoped_tyvars_chirho["aChirho"]),
        outer_chirho
    );
}
