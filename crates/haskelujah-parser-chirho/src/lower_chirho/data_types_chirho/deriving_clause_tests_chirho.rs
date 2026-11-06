// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: deriving clauses keep each class's strategy (row 694).
//!
//! `newtype` is a reserved word, so a clause must consume its keyword token, or the keyword
//! starts a separate declaration named after the class. A declaration may write several
//! clauses, and a class's arguments are never read as classes: the class keeps them.

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, DerivedClassChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::test_support_chirho::parse_and_lower_chirho;

/// One derived class: its written name and its strategy keyword, if any.
type DerivedShapeChirho = (String, Option<String>);

/// One lowered data or newtype declaration: its keyword, name and derived classes.
type DeclShapeChirho = (&'static str, String, Vec<DerivedShapeChirho>);

fn derived_chirho(class_chirho: &str, strategy_chirho: Option<&str>) -> DerivedShapeChirho {
    (
        class_chirho.to_string(),
        strategy_chirho.map(str::to_string),
    )
}

fn derived_shape_chirho(derived_chirho: &DerivedClassChirho) -> DerivedShapeChirho {
    (
        derived_chirho.class_text_chirho().to_string(),
        derived_chirho
            .strategy_chirho
            .as_ref()
            .map(|strategy_chirho| strategy_chirho.keyword_chirho().text_chirho().to_string()),
    )
}

/// Lower a module and keep its data and newtype declarations, in order.
fn decl_shapes_chirho(source_chirho: &str) -> Vec<DeclShapeChirho> {
    parse_and_lower_chirho(source_chirho)
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| {
            let (keyword_chirho, name_chirho, deriving_chirho) = match decl_chirho {
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    deriving_chirho,
                    ..
                } => ("newtype", name_chirho, deriving_chirho),
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    deriving_chirho,
                    ..
                } => ("data", name_chirho, deriving_chirho),
                _ => return None,
            };
            Some((
                keyword_chirho,
                name_chirho.text_chirho().to_string(),
                deriving_chirho.iter().map(derived_shape_chirho).collect(),
            ))
        })
        .collect()
}

#[test]
fn a_bare_newtype_strategy_keeps_one_declaration_chirho() {
    let shapes_chirho = decl_shapes_chirho(
        "module M where\nnewtype Age = Age Int\n  deriving newtype Num\ndata After = After\n",
    );
    assert_eq!(
        shapes_chirho,
        [
            (
                "newtype",
                "Age".to_string(),
                vec![derived_chirho("Num", Some("newtype"))]
            ),
            ("data", "After".to_string(), vec![]),
        ]
    );
}

#[test]
fn a_parenthesized_list_gives_every_class_the_strategy_chirho() {
    let shapes_chirho =
        decl_shapes_chirho("module M where\nnewtype Age = Age Int deriving newtype (Show, Num)\n");
    assert_eq!(
        shapes_chirho,
        [(
            "newtype",
            "Age".to_string(),
            vec![
                derived_chirho("Show", Some("newtype")),
                derived_chirho("Num", Some("newtype")),
            ]
        )]
    );
}

#[test]
fn several_clauses_keep_their_own_strategies_chirho() {
    let shapes_chirho = decl_shapes_chirho(
        "module M where\nnewtype Age = Age Int\n  deriving stock Eq\n  deriving newtype (Show, Num)\n  deriving Ord\ndata After = After deriving anyclass C\n",
    );
    assert_eq!(
        shapes_chirho,
        [
            (
                "newtype",
                "Age".to_string(),
                vec![
                    derived_chirho("Eq", Some("stock")),
                    derived_chirho("Show", Some("newtype")),
                    derived_chirho("Num", Some("newtype")),
                    derived_chirho("Ord", None),
                ]
            ),
            (
                "data",
                "After".to_string(),
                vec![derived_chirho("C", Some("anyclass"))]
            ),
        ]
    );
}

#[test]
fn class_arguments_are_not_classes_chirho() {
    let shapes_chirho = decl_shapes_chirho(
        "module M where\ndata T = T deriving anyclass (C (Maybe Int), D Bool)\ndata After = After\n",
    );
    assert_eq!(
        shapes_chirho,
        [
            (
                "data",
                "T".to_string(),
                vec![
                    derived_chirho("C", Some("anyclass")),
                    derived_chirho("D", Some("anyclass")),
                ]
            ),
            ("data", "After".to_string(), vec![]),
        ]
    );
}

/// A type as a test reads it: constructors and variables by their full names, an application
/// in parentheses, written parentheses dropped.
fn type_text_chirho(type_chirho: &TypeChirho) -> String {
    match type_chirho {
        TypeChirho::ConChirho(name_chirho) | TypeChirho::VarChirho(name_chirho) => {
            name_chirho.full_name_chirho()
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => type_text_chirho(inner_chirho),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} {})",
            type_text_chirho(fun_chirho),
            type_text_chirho(arg_chirho)
        ),
        other_chirho => format!("{other_chirho:?}"),
    }
}

/// Each derived class of the first data or newtype declaration: its full written name and
/// its arguments.
fn applied_classes_chirho(source_chirho: &str) -> Vec<(String, Vec<String>)> {
    parse_and_lower_chirho(source_chirho)
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho {
                deriving_chirho, ..
            }
            | DeclChirho::NewtypeDeclChirho {
                deriving_chirho, ..
            } => Some(deriving_chirho),
            _ => None,
        })
        .expect("the declaration lowers")
        .iter()
        .map(|derived_chirho| {
            (
                derived_chirho.class_chirho.full_name_chirho(),
                derived_chirho
                    .class_arguments_chirho
                    .iter()
                    .map(type_text_chirho)
                    .collect(),
            )
        })
        .collect()
}

#[test]
fn a_class_keeps_its_arguments_in_written_order_chirho() {
    assert_eq!(
        applied_classes_chirho(
            "module M where\ndata T = T deriving anyclass (C (Maybe Int), D Bool a, E)\n"
        ),
        [
            ("C".to_string(), vec!["(Maybe Int)".to_string()]),
            ("D".to_string(), vec!["Bool".to_string(), "a".to_string()]),
            ("E".to_string(), vec![]),
        ]
    );
}

#[test]
fn a_qualified_class_keeps_its_qualifier_chirho() {
    assert_eq!(
        applied_classes_chirho("module M where\ndata T = T deriving stock (P.Eq, P.Show)\n"),
        [("P.Eq".to_string(), vec![]), ("P.Show".to_string(), vec![]),]
    );
    assert_eq!(
        applied_classes_chirho("module M where\ndata T = T deriving P.Ord\n"),
        [("P.Ord".to_string(), vec![])]
    );
}

#[test]
fn an_unwritten_strategy_stays_unspecified_chirho() {
    let shapes_chirho =
        decl_shapes_chirho("module M where\ndata Color = Red | Green deriving (Eq, Show)\n");
    assert_eq!(
        shapes_chirho,
        [(
            "data",
            "Color".to_string(),
            vec![derived_chirho("Eq", None), derived_chirho("Show", None)]
        )]
    );
}

#[test]
fn a_newtype_strategy_on_data_is_kept_for_validation_chirho() {
    // GHC-10333 rejects this; lowering keeps exactly what was written.
    let shapes_chirho =
        decl_shapes_chirho("module M where\ndata Bad = Bad Int deriving newtype Show\n");
    assert_eq!(
        shapes_chirho,
        [(
            "data",
            "Bad".to_string(),
            vec![derived_chirho("Show", Some("newtype"))]
        )]
    );
}

#[test]
fn a_via_clause_stays_in_the_via_record_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nnewtype Age = Age Int\n  deriving stock Eq\n  deriving (Num) via Int\n",
    );
    let shapes_chirho: Vec<DerivedShapeChirho> = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                deriving_chirho, ..
            } => Some(deriving_chirho.iter().map(derived_shape_chirho).collect()),
            _ => None,
        })
        .expect("the newtype declaration lowers");
    assert_eq!(shapes_chirho, [derived_chirho("Eq", Some("stock"))]);
    let via_chirho: Vec<(String, String)> = module_chirho
        .deriving_via_chirho
        .iter()
        .map(|(type_chirho, class_chirho, _)| {
            (
                type_chirho.text_chirho().to_string(),
                class_chirho.text_chirho().to_string(),
            )
        })
        .collect();
    assert_eq!(via_chirho, [("Age".to_string(), "Num".to_string())]);
}

/// The strategy, class and target of every standalone deriving declaration, in order.
fn standalone_shapes_chirho(source_chirho: &str) -> Vec<(Option<String>, String, Vec<String>)> {
    parse_and_lower_chirho(source_chirho)
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::StandaloneDerivingDeclChirho {
                strategy_chirho,
                class_chirho,
                types_chirho,
                ..
            } => Some((
                strategy_chirho.as_ref().map(|strategy_chirho| {
                    strategy_chirho.keyword_chirho().text_chirho().to_string()
                }),
                class_chirho.text_chirho().to_string(),
                types_chirho
                    .iter()
                    .map(|type_chirho| match type_chirho {
                        TypeChirho::ConChirho(name_chirho) => name_chirho.text_chirho().to_string(),
                        other_chirho => format!("{other_chirho:?}"),
                    })
                    .collect(),
            )),
            _ => None,
        })
        .collect()
}

#[test]
fn standalone_deriving_keeps_its_strategy_chirho() {
    let shapes_chirho = standalone_shapes_chirho(
        "module M where\nnewtype Age = Age Int\nderiving newtype instance Show Age\nderiving stock instance Eq Age\nderiving instance Ord Age\n",
    );
    assert_eq!(
        shapes_chirho,
        [
            (
                Some("newtype".to_string()),
                "Show".to_string(),
                vec!["Age".to_string()]
            ),
            (
                Some("stock".to_string()),
                "Eq".to_string(),
                vec!["Age".to_string()]
            ),
            (None, "Ord".to_string(), vec!["Age".to_string()]),
        ]
    );
}
