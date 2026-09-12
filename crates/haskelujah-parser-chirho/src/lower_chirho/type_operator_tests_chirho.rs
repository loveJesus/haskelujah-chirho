// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn signature_chirho(type_text_chirho: &str) -> TypeChirho {
    let source_chirho = format!(
        "{{-# LANGUAGE TypeOperators #-}}\nmodule OperatorChirho where\nprobeChirho :: {type_text_chirho}\nprobeChirho = probeChirho\n"
    );
    signature_in_module_chirho(&source_chirho)
}

fn signature_in_module_chirho(source_chirho: &str) -> TypeChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    module_chirho
        .decls_chirho
        .into_iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "probeChirho" => Some(ty_chirho),
            _ => None,
        })
        .expect("the named signature must survive lowering")
}

fn flat_context_operand_chirho(type_text_chirho: &str, fixities_chirho: &str) -> TypeChirho {
    let source_chirho = format!(
        "{{-# LANGUAGE TypeOperators #-}}\nmodule OperatorChirho where\n{fixities_chirho}class ({type_text_chirho}) ~ resultChirho => EqualChirho resultChirho\n"
    );
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(&source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    module_chirho
        .decls_chirho
        .into_iter()
        .find_map(|decl_chirho| {
            let DeclChirho::ClassDeclChirho { context_chirho, .. } = decl_chirho else {
                return None;
            };
            let [
                ConstraintChirho::ClassChirho {
                    class_chirho,
                    args_chirho,
                    ..
                },
            ] = context_chirho.as_slice()
            else {
                return None;
            };
            assert_eq!(class_chirho.text_chirho(), "~", "{context_chirho:?}");
            args_chirho.first().cloned()
        })
        .expect("the flat superclass must preserve its complete left operand")
}

#[test]
fn flat_superclass_equality_groups_the_family_operand_chirho() {
    let source_chirho = "{-# LANGUAGE TypeOperators, MultiParamTypeClasses #-}\nmodule OperatorChirho where\nclass leftChirho :++ itemChirho ~ resultChirho => PushChirho leftChirho itemChirho resultChirho\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let context_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| {
            if let DeclChirho::ClassDeclChirho { context_chirho, .. } = decl_chirho {
                Some(context_chirho)
            } else {
                None
            }
        })
        .expect("the class and its superclass constraint survive lowering");
    let [
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        },
    ] = context_chirho.as_slice()
    else {
        panic!("expected one equality constraint, got {context_chirho:?}");
    };
    assert_eq!(class_chirho.text_chirho(), "~", "{context_chirho:?}");
    assert_eq!(args_chirho.len(), 2);
    let (operator_chirho, _, _) = application_chirho(&args_chirho[0]);
    assert!(
        matches!(operator_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == ":++"),
        "{context_chirho:?}"
    );
}

#[test]
fn signature_equality_groups_the_promoted_cons_operand_chirho() {
    let ty_chirho = signature_chirho("(xsChirho ~ Int ': restChirho) => Int");
    let TypeChirho::QualChirho { context_chirho, .. } = ty_chirho else {
        panic!("expected a retained context, got {ty_chirho:?}");
    };
    let [
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        },
    ] = context_chirho.as_slice()
    else {
        panic!("expected one equality constraint, got {context_chirho:?}");
    };
    assert_eq!(class_chirho.text_chirho(), "~", "{context_chirho:?}");
    assert_eq!(args_chirho.len(), 2);
    let (operator_chirho, _, _) = application_chirho(&args_chirho[1]);
    assert!(
        matches!(operator_chirho, TypeChirho::PromotedConChirho { name_chirho, .. } if name_chirho.text_chirho() == ":"),
        "{context_chirho:?}"
    );
}

#[test]
fn infix_type_fixity_and_parentheses_determine_operand_grouping_chirho() {
    for (signature_text_chirho, fixities_chirho, root_chirho, nested_side_chirho, nested_chirho) in [
        (
            "Int :*: Bool :+: Char",
            "infixl 4 :*:\ninfixl 3 :+:\n",
            ":+:",
            0,
            ":*:",
        ),
        (
            "Int :+: Bool :*: Char",
            "infixl 4 :*:\ninfixl 3 :+:\n",
            ":+:",
            1,
            ":*:",
        ),
        ("Int :+: Bool :+: Char", "infixr 3 :+:\n", ":+:", 1, ":+:"),
        ("Int :+: Bool :+: Char", "", ":+:", 0, ":+:"),
        (
            "Int :*: (Bool :+: Char)",
            "infixl 4 :*:\ninfixl 3 :+:\n",
            ":*:",
            1,
            ":+:",
        ),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeOperators #-}}\nmodule OperatorChirho where\n{fixities_chirho}probeChirho :: {signature_text_chirho}\nprobeChirho = probeChirho\n"
        );
        for ty_chirho in [
            signature_in_module_chirho(&source_chirho),
            flat_context_operand_chirho(signature_text_chirho, fixities_chirho),
        ] {
            let (operator_chirho, left_chirho, right_chirho) = application_chirho(&ty_chirho);
            assert!(
                matches!(operator_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == root_chirho),
                "{signature_text_chirho}: {ty_chirho:?}"
            );
            let (nested_operator_chirho, _, _) = application_chirho(if nested_side_chirho == 0 {
                left_chirho
            } else {
                right_chirho
            });
            assert!(
                matches!(nested_operator_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == nested_chirho),
                "{signature_text_chirho}: {ty_chirho:?}"
            );
        }
    }
}

fn application_chirho(ty_chirho: &TypeChirho) -> (&TypeChirho, &TypeChirho, &TypeChirho) {
    if let TypeChirho::ParenChirho { inner_chirho, .. } = ty_chirho {
        return application_chirho(inner_chirho);
    }
    let TypeChirho::AppChirho {
        fun_chirho,
        arg_chirho: right_chirho,
        ..
    } = ty_chirho
    else {
        panic!("expected binary type application, got {ty_chirho:?}");
    };
    let TypeChirho::AppChirho {
        fun_chirho: operator_chirho,
        arg_chirho: left_chirho,
        ..
    } = fun_chirho.as_ref()
    else {
        panic!("expected the operator applied to its left operand, got {fun_chirho:?}");
    };
    (operator_chirho, left_chirho, right_chirho)
}

#[test]
fn infix_variable_preserves_namespace_and_source_span_chirho() {
    let ty_chirho = signature_chirho("leftChirho `operatorChirho` rightChirho");
    let (operator_chirho, left_chirho, right_chirho) = application_chirho(&ty_chirho);
    assert!(
        matches!(operator_chirho, TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "operatorChirho"),
        "{operator_chirho:?}"
    );
    assert!(
        matches!(left_chirho, TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "leftChirho")
    );
    assert!(
        matches!(right_chirho, TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "rightChirho")
    );
    assert!(
        left_chirho.span_chirho().start_chirho() < operator_chirho.span_chirho().start_chirho()
    );
    assert!(
        operator_chirho.span_chirho().start_chirho() < right_chirho.span_chirho().start_chirho()
    );
}

#[test]
fn flat_infix_names_keep_their_namespace_and_operator_span_chirho() {
    for (text_chirho, name_text_chirho, variable_chirho, promoted_chirho) in [
        (
            "leftChirho `operatorChirho` rightChirho",
            "operatorChirho",
            true,
            false,
        ),
        ("leftChirho `Either` rightChirho", "Either", false, false),
        ("leftChirho ': rightChirho", ":", false, true),
    ] {
        let ty_chirho = flat_context_operand_chirho(text_chirho, "");
        let (operator_chirho, left_chirho, right_chirho) = application_chirho(&ty_chirho);
        match operator_chirho {
            TypeChirho::VarChirho(name_chirho) if variable_chirho => {
                assert_eq!(name_chirho.text_chirho(), name_text_chirho)
            }
            TypeChirho::PromotedConChirho { name_chirho, .. } if promoted_chirho => {
                assert_eq!(name_chirho.text_chirho(), name_text_chirho)
            }
            TypeChirho::ConChirho(name_chirho) if !variable_chirho && !promoted_chirho => {
                assert_eq!(name_chirho.text_chirho(), name_text_chirho)
            }
            other_chirho => panic!("incorrect namespace for {text_chirho}: {other_chirho:?}"),
        }
        assert!(
            left_chirho.span_chirho().end_chirho() <= operator_chirho.span_chirho().start_chirho()
        );
        assert!(
            operator_chirho.span_chirho().end_chirho() <= right_chirho.span_chirho().start_chirho()
        );
    }
}

#[test]
fn infix_constructor_names_remain_constructors_chirho() {
    for operator_name_chirho in ["Either", "Data.Either.Either", ":*:", "~>"] {
        let spelling_chirho = if operator_name_chirho
            .chars()
            .next()
            .is_some_and(char::is_alphabetic)
        {
            format!("Int `{operator_name_chirho}` Bool")
        } else {
            format!("Int {operator_name_chirho} Bool")
        };
        let ty_chirho = signature_chirho(&spelling_chirho);
        let (operator_chirho, _, _) = application_chirho(&ty_chirho);
        assert!(
            matches!(operator_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.full_name_chirho() == operator_name_chirho),
            "{operator_name_chirho}: {operator_chirho:?}"
        );
    }
}

#[test]
fn infix_constructor_field_uses_its_declared_type_variable_chirho() {
    let source_chirho = "{-# LANGUAGE TypeOperators #-}\nmodule FieldChirho where\ndata FieldChirho leftChirho operatorChirho = FieldChirho (leftChirho `operatorChirho` Int)\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let field_chirho =
        module_chirho
            .decls_chirho
            .iter()
            .find_map(|decl_chirho| match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho.iter().find_map(|constructor_chirho| {
                    match constructor_chirho {
                        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                            fields_chirho.first()
                        }
                        _ => None,
                    }
                }),
                _ => None,
            })
            .expect("the constructor must retain its field");
    let (operator_chirho, _, _) = application_chirho(&field_chirho.1);
    assert!(
        matches!(operator_chirho, TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "operatorChirho"),
        "{operator_chirho:?}"
    );
}
