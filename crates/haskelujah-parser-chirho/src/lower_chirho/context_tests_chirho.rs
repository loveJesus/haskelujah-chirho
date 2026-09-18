// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Context retention is an AST contract: no later phase can recover a
//! constraint, an argument or a binder the lowering dropped, and nothing in
//! the AST says that one was dropped.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

/// Shape without spans or parentheses; variables are marked so a variable
/// can never be confused with a constructor of the same spelling.
fn type_shape_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => name_chirho.text_chirho().to_owned(),
        TypeChirho::VarChirho(name_chirho) => format!("var({})", name_chirho.text_chirho()),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} {})",
            type_shape_chirho(fun_chirho),
            type_shape_chirho(arg_chirho)
        ),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", type_shape_chirho(element_chirho))
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => format!(
            "({} -> {})",
            type_shape_chirho(arg_chirho),
            type_shape_chirho(result_chirho)
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => format!(
            "({})",
            elements_chirho
                .iter()
                .map(type_shape_chirho)
                .collect::<Vec<_>>()
                .join(",")
        ),
        TypeChirho::ParenChirho { inner_chirho, .. } => type_shape_chirho(inner_chirho),
        other_chirho => panic!("unexpected type in a context: {other_chirho:?}"),
    }
}

fn constraint_shape_chirho(constraint_chirho: &ConstraintChirho) -> String {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => std::iter::once(class_chirho.text_chirho().to_owned())
            .chain(args_chirho.iter().map(type_shape_chirho))
            .collect::<Vec<_>>()
            .join(" "),
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => format!(
            "forall {}. [{}] => {}",
            vars_chirho
                .iter()
                .map(|var_chirho| match &var_chirho.kind_annotation_chirho {
                    Some(_) => format!("{}::kinded", var_chirho.name_chirho.text_chirho()),
                    None => var_chirho.name_chirho.text_chirho().to_owned(),
                })
                .collect::<Vec<_>>()
                .join(" "),
            context_shape_chirho(context_chirho),
            constraint_shape_chirho(body_chirho)
        ),
    }
}

fn context_shape_chirho(context_chirho: &[ConstraintChirho]) -> String {
    context_chirho
        .iter()
        .map(constraint_shape_chirho)
        .collect::<Vec<_>>()
        .join("; ")
}

/// What one declaration lowered to: its context, the class it names, and the
/// head types it applies the class to (empty for a class declaration).
struct LoweredChirho {
    context_chirho: String,
    class_chirho: String,
    head_chirho: Vec<String>,
}

fn lower_declaration_chirho(declaration_chirho: &str) -> LoweredChirho {
    let source_chirho = format!(
        "{{-# LANGUAGE QuantifiedConstraints, DataKinds, KindSignatures, TypeFamilies #-}}\nmodule ContextChirho where\n{declaration_chirho}\n"
    );
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(&source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    for lowered_chirho in &module_chirho.decls_chirho {
        match lowered_chirho {
            DeclChirho::ClassDeclChirho {
                context_chirho,
                name_chirho,
                ..
            } => {
                return LoweredChirho {
                    context_chirho: context_shape_chirho(context_chirho),
                    class_chirho: name_chirho.text_chirho().to_owned(),
                    head_chirho: Vec::new(),
                };
            }
            DeclChirho::InstanceDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                ..
            }
            | DeclChirho::StandaloneDerivingDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                ..
            } => {
                return LoweredChirho {
                    context_chirho: context_shape_chirho(context_chirho),
                    class_chirho: class_chirho.text_chirho().to_owned(),
                    head_chirho: types_chirho.iter().map(type_shape_chirho).collect(),
                };
            }
            _ => {}
        }
    }
    panic!("no class, instance or standalone deriving declaration was lowered");
}

fn check_contexts_chirho(cases_chirho: &[(&str, &str)]) {
    for &(declaration_chirho, expected_chirho) in cases_chirho {
        assert_eq!(
            lower_declaration_chirho(declaration_chirho).context_chirho,
            expected_chirho,
            "{declaration_chirho}"
        );
    }
}

#[test]
fn every_member_of_a_context_tuple_is_kept_chirho() {
    check_contexts_chirho(&[
        (
            "class (BChirho d, CChirho d) => DChirho d",
            "BChirho var(d); CChirho var(d)",
        ),
        (
            "instance (Eq a, Show a) => CChirho (BoxChirho a)",
            "Eq var(a); Show var(a)",
        ),
        (
            "deriving instance (Show a, Eq a) => Show (BoxChirho a)",
            "Show var(a); Eq var(a)",
        ),
        ("instance (Eq a) => CChirho (BoxChirho a)", "Eq var(a)"),
        ("instance Eq a => CChirho (BoxChirho a)", "Eq var(a)"),
    ]);
}

#[test]
fn concrete_and_multiple_arguments_are_kept_in_order_chirho() {
    check_contexts_chirho(&[
        (
            "instance C1Chirho x T1Chirho => C2Chirho x T1Chirho",
            "C1Chirho var(x) T1Chirho",
        ),
        (
            "class (ReaderChirho r m, StateChirho s m) => StackChirho r s m",
            "ReaderChirho var(r) var(m); StateChirho var(s) var(m)",
        ),
        (
            "instance (MulChirho a b c, AddChirho c Int d) => MulChirho a [b] d",
            "MulChirho var(a) var(b) var(c); AddChirho var(c) Int var(d)",
        ),
    ]);
}

#[test]
fn an_argument_that_is_an_application_stays_one_argument_chirho() {
    check_contexts_chirho(&[
        (
            "instance DuperChirho (FamChirho a) => SuperChirho a",
            "DuperChirho (FamChirho var(a))",
        ),
        (
            "class (CChirho (f a), DChirho b) => EChirho f a b",
            "CChirho (var(f) var(a)); DChirho var(b)",
        ),
        (
            "instance (Show (f (FixChirho f))) => Show (FixChirho f)",
            "Show (var(f) (FixChirho var(f)))",
        ),
        ("instance Eq [a] => CChirho (BoxChirho a)", "Eq [var(a)]"),
    ]);
}

#[test]
fn equalities_and_synonym_applications_keep_both_sides_chirho() {
    check_contexts_chirho(&[
        ("instance a ~ Int => CChirho [a]", "~ var(a) Int"),
        (
            "instance (FamChirho a ~ b, Eq b) => CChirho (BoxChirho a)",
            "~ (FamChirho var(a)) var(b); Eq var(b)",
        ),
        (
            "class (FamChirho a ~ Bool) => GuardedChirho a",
            "~ (FamChirho var(a)) Bool",
        ),
        (
            "instance StringyChirho a => CChirho (BoxChirho a)",
            "StringyChirho var(a)",
        ),
    ]);
}

#[test]
fn a_kind_annotated_argument_is_the_annotated_type_chirho() {
    check_contexts_chirho(&[
        (
            "instance KnownChirho (n :: Nat) => CChirho (ProxyChirho n)",
            "KnownChirho var(n)",
        ),
        (
            "class (KnownChirho (n :: Nat), Eq a) => SizedChirho n a",
            "KnownChirho var(n); Eq var(a)",
        ),
    ]);
}

#[test]
fn an_instance_forall_binds_variables_and_is_not_a_quantified_given_chirho() {
    let with_context_chirho = lower_declaration_chirho("instance forall a. Eq a => CChirho [a]");
    assert_eq!(with_context_chirho.context_chirho, "Eq var(a)");
    assert_eq!(with_context_chirho.class_chirho, "CChirho");
    assert_eq!(with_context_chirho.head_chirho, ["[var(a)]"]);

    let kinded_chirho =
        lower_declaration_chirho("instance forall (a :: Type) b. (Eq a, Eq b) => CChirho (a, b)");
    assert_eq!(kinded_chirho.context_chirho, "Eq var(a); Eq var(b)");
    assert_eq!(kinded_chirho.class_chirho, "CChirho");

    // Without a context the telescope sits in front of the head: the class
    // must still be the class, not a type argument of a nameless instance.
    let without_context_chirho = lower_declaration_chirho("instance forall a. CChirho [a]");
    assert_eq!(without_context_chirho.context_chirho, "");
    assert_eq!(without_context_chirho.class_chirho, "CChirho");
    assert_eq!(without_context_chirho.head_chirho, ["[var(a)]"]);

    let deriving_chirho =
        lower_declaration_chirho("deriving instance forall a. Show a => Show (BoxChirho a)");
    assert_eq!(deriving_chirho.context_chirho, "Show var(a)");
    assert_eq!(deriving_chirho.class_chirho, "Show");
}

#[test]
fn a_quantified_given_keeps_binders_premise_and_conclusion_chirho() {
    check_contexts_chirho(&[
        (
            "instance (forall b. Eq b => Eq (f b)) => Eq (RoseChirho f)",
            "forall b. [Eq var(b)] => Eq (var(f) var(b))",
        ),
        (
            "class (forall b. Eq b => Eq (f b)) => LiftedChirho f",
            "forall b. [Eq var(b)] => Eq (var(f) var(b))",
        ),
        (
            "class (forall (b :: Type). Eq b => CChirho f b) => DChirho f",
            "forall b::kinded. [Eq var(b)] => CChirho var(f) var(b)",
        ),
        (
            "class (forall b. CChirho (f b)) => AlwaysChirho f",
            "forall b. [] => CChirho (var(f) var(b))",
        ),
        (
            "instance (Eq a, forall b. Eq b => Eq (f b)) => Eq (NodeChirho f a)",
            "Eq var(a); forall b. [Eq var(b)] => Eq (var(f) var(b))",
        ),
        (
            "deriving instance (forall b. Show b => Show (f b)) => Show (RoseChirho f)",
            "forall b. [Show var(b)] => Show (var(f) var(b))",
        ),
    ]);
}

#[test]
fn a_quantified_given_does_not_move_the_declaration_arrow_chirho() {
    // The `=>` inside the parentheses belongs to the quantified constraint.
    let instance_chirho =
        lower_declaration_chirho("instance (forall b. Eq b => Eq (f b)) => Eq (RoseChirho f)");
    assert_eq!(instance_chirho.class_chirho, "Eq");
    assert_eq!(instance_chirho.head_chirho, ["(RoseChirho var(f))"]);

    let deriving_chirho = lower_declaration_chirho(
        "deriving instance (forall b. Show b => Show (f b)) => Show (RoseChirho f)",
    );
    assert_eq!(deriving_chirho.class_chirho, "Show");
}
