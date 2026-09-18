// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn types_chirho(type_text_chirho: &str, gadt_chirho: bool) -> (TypeChirho, TypeChirho) {
    let declaration_chirho = if gadt_chirho {
        format!(
            "data HolderChirho where\n  HolderChirho :: {{ fieldChirho :: {type_text_chirho} }} -> HolderChirho"
        )
    } else {
        format!("data HolderChirho = HolderChirho {{ fieldChirho :: {type_text_chirho} }}")
    };
    let source_chirho = format!(
        "{{-# LANGUAGE GADTs, TypeOperators #-}}\nmodule FlatTypeChirho where\n{declaration_chirho}\nprobeChirho :: {type_text_chirho}\nprobeChirho = probeChirho\n"
    );
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(&source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let mut field_chirho = None;
    let mut signature_chirho = None;
    for declaration_chirho in module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => {
                for constructor_chirho in constructors_chirho {
                    if let ConDeclChirho::RecordChirho { fields_chirho, .. } = constructor_chirho {
                        field_chirho = fields_chirho
                            .into_iter()
                            .next()
                            .map(|field_chirho| field_chirho.ty_chirho);
                    }
                }
            }
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == "probeChirho" => signature_chirho = Some(ty_chirho),
            _ => {}
        }
    }
    (
        field_chirho.expect("record field must survive"),
        signature_chirho.expect("signature following the record must survive"),
    )
}

// Semantic type shape is the parser's contract here, not temporary Core/IR layout.
// Ignore spans and parentheses while retaining constructor/application distinction.
fn shape_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => name_chirho.text_chirho().to_owned(),
        TypeChirho::VarChirho(name_chirho) => format!("var({})", name_chirho.text_chirho()),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} {})",
            shape_chirho(fun_chirho),
            shape_chirho(arg_chirho)
        ),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", shape_chirho(element_chirho))
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => format!(
            "({} -> {})",
            shape_chirho(arg_chirho),
            shape_chirho(result_chirho)
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => format!(
            "({})",
            elements_chirho
                .iter()
                .map(shape_chirho)
                .collect::<Vec<_>>()
                .join(",")
        ),
        TypeChirho::ParenChirho { inner_chirho, .. } => shape_chirho(inner_chirho),
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => format!(
            "forall {}. {}",
            vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_owned())
                .collect::<Vec<_>>()
                .join(" "),
            shape_chirho(body_chirho)
        ),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => format!(
            "[{}] => {}",
            context_chirho
                .iter()
                .map(|constraint_chirho| {
                    std::iter::once(
                        constraint_chirho
                            .simple_class_chirho()
                            .expect("these cases use class constraints only")
                            .text_chirho()
                            .to_owned(),
                    )
                    .chain(
                        constraint_chirho
                            .simple_args_chirho()
                            .expect("these cases use class constraints only")
                            .iter()
                            .map(shape_chirho),
                    )
                    .collect::<Vec<_>>()
                    .join(" ")
                })
                .collect::<Vec<_>>()
                .join("; "),
            shape_chirho(body_chirho)
        ),
        other_chirho => panic!("unexpected type shape: {other_chirho:?}"),
    }
}

fn check_shapes_chirho(cases_chirho: &[(&str, &str)]) {
    for &(text_chirho, expected_chirho) in cases_chirho {
        for gadt_chirho in [false, true] {
            let (field_chirho, signature_chirho) = types_chirho(text_chirho, gadt_chirho);
            assert_eq!(
                shape_chirho(&field_chirho),
                expected_chirho,
                "field {text_chirho}, GADT={gadt_chirho}"
            );
            assert_eq!(
                shape_chirho(&signature_chirho),
                expected_chirho,
                "signature {text_chirho}, GADT={gadt_chirho}"
            );
        }
    }
}

#[test]
fn empty_brackets_remain_a_constructor_without_fabricated_element_chirho() {
    check_shapes_chirho(&[
        ("[]", "[]"),
        ("[] Char", "([] Char)"),
        ("([]) Char", "([] Char)"),
        ("Maybe []", "(Maybe [])"),
    ]);
}

#[test]
fn nested_brackets_preserve_the_inner_function_tail_chirho() {
    check_shapes_chirho(&[
        ("[[Int] -> Int]", "[([Int] -> Int)]"),
        ("[[[Int] -> Int] -> Bool]", "[([([Int] -> Int)] -> Bool)]"),
    ]);
}

#[test]
fn list_syntax_and_prefix_application_remain_distinct_chirho() {
    check_shapes_chirho(&[
        ("[Char]", "[Char]"),
        ("[[Char]]", "[[Char]]"),
        ("[[] Char]", "[([] Char)]"),
        ("[] (Maybe Char)", "([] (Maybe Char))"),
    ]);
}

#[test]
fn nested_list_tuple_and_outer_function_boundaries_survive_chirho() {
    check_shapes_chirho(&[
        ("[([Char], Maybe [Bool])]", "[([Char],(Maybe [Bool]))]"),
        ("[[Int] -> Int] -> [Bool]", "([([Int] -> Int)] -> [Bool])"),
    ]);
}

#[test]
fn prefix_list_application_survives_the_operator_context_chirho() {
    fn contains_list_application_chirho(ty_chirho: &TypeChirho) -> bool {
        match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                (matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "[]")
                    && matches!(arg_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Char"))
                    || contains_list_application_chirho(fun_chirho)
                    || contains_list_application_chirho(arg_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                contains_list_application_chirho(inner_chirho)
            }
            _ => false,
        }
    }
    // T14761c's relevant subtree. This control intentionally does not assert
    // the separate flat infix-association contract.
    for gadt_chirho in [false, true] {
        let (field_chirho, signature_chirho) =
            types_chirho("Maybe Int && [] Char && Int", gadt_chirho);
        assert!(
            contains_list_application_chirho(&field_chirho),
            "{field_chirho:?}"
        );
        assert!(
            contains_list_application_chirho(&signature_chirho),
            "{signature_chirho:?}"
        );
    }
}

// A forall extends as far to the right as possible. The flat grammar used to
// split on `=>` and `->` first, so the quantifier covered only the first
// argument. The structured signature path is the agreement oracle here.
// Workflow: language-features-chirho/flat-type-syntax-chirho.
#[test]
fn a_leading_forall_scopes_over_arrows_and_contexts_chirho() {
    check_shapes_chirho(&[
        ("forall a. a -> a", "forall a. (var(a) -> var(a))"),
        (
            "forall a. Eq a => a -> Bool",
            "forall a. [Eq var(a)] => (var(a) -> Bool)",
        ),
        (
            "(forall a. a -> a) -> Int",
            "(forall a. (var(a) -> var(a)) -> Int)",
        ),
        (
            "forall a. a -> forall b. b -> a",
            "forall a. (var(a) -> forall b. (var(b) -> var(a)))",
        ),
    ]);
}

// `(t :: k)` is the type `t`: main keeps no kind annotation on a type, and the
// flat grammar used to read the annotation as one more argument.
#[test]
fn a_kind_annotation_is_not_an_argument_chirho() {
    check_shapes_chirho(&[
        ("Maybe (a :: Type)", "(Maybe var(a))"),
        ("Either (a :: Type) (b :: Type)", "((Either var(a)) var(b))"),
    ]);
}
