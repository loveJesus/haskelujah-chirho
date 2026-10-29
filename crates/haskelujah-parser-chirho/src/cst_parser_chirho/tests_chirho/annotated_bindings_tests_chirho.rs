// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A typed pattern followed by `=` still binds its pattern and retains its right-hand side.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{LocalBindChirho, RhsChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};

use super::ParserChirho;
use crate::lower_chirho::lower_module_chirho;

fn count_nodes_chirho(node_chirho: &GreenNodeChirho, kind_chirho: SyntaxKindChirho) -> usize {
    usize::from(node_chirho.kind_chirho() == kind_chirho)
        + node_chirho
            .children_chirho()
            .iter()
            .map(|child_chirho| match child_chirho {
                GreenElementChirho::NodeChirho(child_chirho) => {
                    count_nodes_chirho(child_chirho, kind_chirho)
                }
                GreenElementChirho::TokenChirho(_) => 0,
            })
            .sum::<usize>()
}

fn lower_without_errors_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    assert_eq!(
        count_nodes_chirho(&green_chirho, SyntaxKindChirho::ErrorNodeChirho),
        0,
        "{source_chirho}\n{green_chirho:#?}"
    );
    lower_module_chirho(&green_chirho, file_chirho)
}

fn bound_names_chirho(pat_chirho: &PatChirho) -> Vec<&str> {
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => vec![name_chirho.text_chirho()],
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => bound_names_chirho(pat_chirho),
        PatChirho::ParenChirho { inner_chirho, .. } => bound_names_chirho(inner_chirho),
        PatChirho::TupleChirho {
            elements_chirho, ..
        }
        | PatChirho::ListChirho {
            elements_chirho, ..
        } => elements_chirho
            .iter()
            .flat_map(bound_names_chirho)
            .collect(),
        other_chirho => panic!("unexpected pattern: {other_chirho:?}"),
    }
}

#[test]
fn annotated_patterns_with_a_rhs_remain_bindings_chirho() {
    for (binding_chirho, names_chirho) in [
        ("valueChirho :: Int = 7", vec!["valueChirho"]),
        (
            "(leftChirho, rightChirho) :: (Int, Int) = (1, 2)",
            vec!["leftChirho", "rightChirho"],
        ),
        ("[itemChirho] :: [Int] = [1]", vec!["itemChirho"]),
        ("(wrappedChirho) :: Int = 8", vec!["wrappedChirho"]),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE ScopedTypeVariables #-}}\nmodule MChirho where\n{binding_chirho}\n"
        );
        let module_chirho = lower_without_errors_chirho(&source_chirho);
        assert_eq!(module_chirho.decls_chirho.len(), 1, "{source_chirho}");
        let DeclChirho::PatBindChirho {
            pat_chirho,
            rhs_chirho,
            ..
        } = &module_chirho.decls_chirho[0]
        else {
            panic!("lost annotated binding: {module_chirho:?}");
        };
        assert!(matches!(pat_chirho, PatChirho::TypeAnnotChirho { .. }));
        assert_eq!(bound_names_chirho(pat_chirho), names_chirho);
        assert!(matches!(rhs_chirho, RhsChirho::UnguardedChirho(_)));
    }
}

#[test]
fn tc135s_annotated_where_binding_keeps_its_name_and_rhs_chirho() {
    let module_chirho = lower_without_errors_chirho(include_str!(
        "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc135.hs"
    ));
    let matches_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } if name_chirho.text_chirho() == "f" => Some(matches_chirho),
            _ => None,
        })
        .expect("tc135's f");
    assert_eq!(matches_chirho[0].where_binds_chirho.len(), 1);
    let LocalBindChirho::PatBindChirho {
        pat_chirho,
        rhs_chirho,
        ..
    } = &matches_chirho[0].where_binds_chirho[0]
    else {
        panic!("lost t3's binding: {:?}", matches_chirho[0]);
    };
    assert!(matches!(pat_chirho, PatChirho::TypeAnnotChirho { .. }));
    assert_eq!(bound_names_chirho(pat_chirho), ["t3"]);
    assert!(matches!(rhs_chirho, RhsChirho::UnguardedChirho(_)));
}

#[test]
fn standalone_signatures_do_not_borrow_a_later_bindings_equals_chirho() {
    for (source_chirho, count_chirho) in [
        ("valueChirho :: Int\nvalueChirho = 7\n", 1),
        (
            "leftChirho, rightChirho :: Int\nleftChirho = 1\nrightChirho = 2\n",
            2,
        ),
        (
            "identityChirho :: forall a. a -> a\nidentityChirho xChirho = xChirho\n",
            1,
        ),
        (
            "(<+>) :: Int -> Int -> Int\nxChirho <+> yChirho = xChirho + yChirho\n",
            1,
        ),
        (
            "fChirho :: (forall a. a -> a) -> Int\nfChirho gChirho = gChirho 7\n",
            1,
        ),
        (
            "module MChirho where { valueChirho :: Int; valueChirho = 7 }",
            1,
        ),
    ] {
        let module_chirho = lower_without_errors_chirho(source_chirho);
        assert_eq!(
            module_chirho
                .decls_chirho
                .iter()
                .filter(|decl_chirho| matches!(decl_chirho, DeclChirho::TypeSigChirho { .. }))
                .count(),
            count_chirho,
            "{source_chirho}"
        );
        assert!(
            module_chirho
                .decls_chirho
                .iter()
                .all(|decl_chirho| !matches!(decl_chirho, DeclChirho::PatBindChirho { .. })),
            "{source_chirho}"
        );
    }
}
