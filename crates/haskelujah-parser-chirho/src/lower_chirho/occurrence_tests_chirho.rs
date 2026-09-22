// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering stamps every occurrence it builds with an origin of its own, and
//! nothing else. Controls: each reference and operator carries a distinct
//! origin; a guard fall-through that lowering places twice is distinct in each
//! place; the knot of a recursive do stamps the references it introduces; the
//! module's supply continues past everything lowering minted; binders, types
//! and other names carry no origin.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::occurrences_chirho::visit_decl_chirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::provenance_chirho::{OriginIdChirho, OriginSupplyChirho};
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};

use super::{LowerCtxChirho, lower_module_chirho};
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    lower_module_chirho(&cst_chirho, file_chirho)
}

/// Every occurrence in the module as (text, origin), in source order. A
/// literal is written as its value in angle brackets.
fn occurrences_chirho(module_chirho: &mut ModuleChirho) -> Vec<(String, Option<OriginIdChirho>)> {
    use haskelujah_ast_chirho::lit_chirho::LitChirho;
    use haskelujah_ast_chirho::occurrences_chirho::OccurrenceMutChirho;
    let mut found_chirho = Vec::new();
    for decl_chirho in &mut module_chirho.decls_chirho {
        visit_decl_chirho(decl_chirho, &mut |occurrence_chirho| {
            let text_chirho = match &occurrence_chirho {
                OccurrenceMutChirho::ReferenceChirho(name_chirho) => {
                    name_chirho.text_chirho().to_string()
                }
                OccurrenceMutChirho::LiteralChirho(lit_chirho) => match &**lit_chirho {
                    LitChirho::IntChirho(value_chirho, ..) => format!("<{value_chirho}>"),
                    LitChirho::FloatChirho(value_chirho, ..) => format!("<{value_chirho}>"),
                    LitChirho::CharChirho(value_chirho, ..) => format!("<{value_chirho:?}>"),
                    LitChirho::StringChirho(value_chirho, ..) => format!("<{value_chirho:?}>"),
                },
            };
            found_chirho.push((text_chirho, occurrence_chirho.origin_chirho()));
        });
    }
    found_chirho
}

/// Every occurrence has an origin, no two share one, and no name that is not an
/// occurrence carries one. The last is read from the Debug rendering, which
/// names an origin only where a name has one.
fn assert_origins_exactly_at_occurrences_chirho(
    module_chirho: &mut ModuleChirho,
) -> Vec<(String, OriginIdChirho)> {
    let found_chirho = occurrences_chirho(module_chirho);
    let stamped_chirho: Vec<(String, OriginIdChirho)> = found_chirho
        .iter()
        .map(|(text_chirho, origin_chirho)| {
            let origin_chirho =
                origin_chirho.unwrap_or_else(|| panic!("`{text_chirho}` has no origin"));
            (text_chirho.clone(), origin_chirho)
        })
        .collect();
    let distinct_chirho: HashSet<OriginIdChirho> = stamped_chirho
        .iter()
        .map(|(_, origin_chirho)| *origin_chirho)
        .collect();
    assert_eq!(
        distinct_chirho.len(),
        stamped_chirho.len(),
        "{stamped_chirho:?}"
    );
    let rendered_chirho = format!("{:?}", module_chirho.decls_chirho);
    assert_eq!(
        rendered_chirho.matches("OriginIdChirho(").count(),
        stamped_chirho.len(),
        "a name or literal that is not an occurrence carries an origin"
    );
    stamped_chirho
}

fn texts_chirho(stamped_chirho: &[(String, OriginIdChirho)]) -> Vec<&str> {
    stamped_chirho
        .iter()
        .map(|(text_chirho, _)| text_chirho.as_str())
        .collect()
}

#[test]
fn references_and_operators_each_take_their_own_origin_chirho() {
    let mut module_chirho = lower_chirho(
        "module Main where\n\
         addChirho x = x + x\n\
         incChirho = (+ 1)\n\
         halfChirho = (`div` 2)\n\
         justsChirho ms = [y | Just y <- ms]\n\
         data BoxChirho = BoxChirho { boxedChirho :: Int }\n\
         boxChirho = BoxChirho { boxedChirho = 1 }\n\
         wrapChirho = Just 3\n\
         main = print (addChirho 1)\n",
    );
    let stamped_chirho = assert_origins_exactly_at_occurrences_chirho(&mut module_chirho);
    let texts_chirho = texts_chirho(&stamped_chirho);
    // Constructor uses are references too (a constructor can carry a class
    // context); the `Just` of a pattern is not a use and has no origin.
    for expected_chirho in ["x", "+", "div", "print", "addChirho", "BoxChirho"] {
        assert!(texts_chirho.contains(&expected_chirho), "{texts_chirho:?}");
    }
    assert_eq!(
        texts_chirho
            .iter()
            .filter(|text_chirho| **text_chirho == "Just")
            .count(),
        1,
        "{texts_chirho:?}"
    );
    // `x + x` is two uses of one variable and one use of `+`: three occurrences.
    assert_eq!(
        texts_chirho
            .iter()
            .filter(|text_chirho| **text_chirho == "x")
            .count(),
        2
    );
}

#[test]
fn a_fall_through_placed_twice_is_two_occurrences_chirho() {
    // The first guard has two qualifiers, so lowering places the remaining
    // guards once per failing qualifier. Each copy is checked on its own and
    // must be identified on its own.
    let mut module_chirho = lower_chirho(
        "module Main where\n\
         pickChirho m | Just x <- m, x > 0 = x\n\
         \x20           | otherwise = negate 1\n",
    );
    let stamped_chirho = assert_origins_exactly_at_occurrences_chirho(&mut module_chirho);
    let texts_chirho = texts_chirho(&stamped_chirho);
    let otherwise_uses_chirho = texts_chirho
        .iter()
        .filter(|text_chirho| **text_chirho == "otherwise")
        .count();
    assert!(otherwise_uses_chirho >= 2, "{texts_chirho:?}");
}

#[test]
fn a_recursive_do_knot_stamps_the_references_it_introduces_chirho() {
    let mut module_chirho = lower_chirho(
        "{-# LANGUAGE RecursiveDo #-}\n\
         module Main where\n\
         main = mdo\n\
         \x20 xs <- return (1 : ys)\n\
         \x20 ys <- return (2 : xs)\n\
         \x20 print (take 3 xs)\n",
    );
    let stamped_chirho = assert_origins_exactly_at_occurrences_chirho(&mut module_chirho);
    let texts_chirho = texts_chirho(&stamped_chirho);
    assert!(texts_chirho.contains(&"mfix"), "{texts_chirho:?}");
    // Two `return`s written by the user and one the knot adds.
    assert_eq!(
        texts_chirho
            .iter()
            .filter(|text_chirho| **text_chirho == "return")
            .count(),
        3
    );
}

#[test]
fn the_module_supply_continues_past_lowering_chirho() {
    let mut module_chirho = lower_chirho(
        "module Main where\n\
         main = print (length [1, 2, 3])\n",
    );
    let stamped_chirho = assert_origins_exactly_at_occurrences_chirho(&mut module_chirho);
    let next_chirho = module_chirho.origin_supply_chirho.fresh_chirho();
    assert!(
        stamped_chirho
            .iter()
            .all(|(_, origin_chirho)| *origin_chirho != next_chirho),
        "a later producer would reuse an origin lowering already minted"
    );
}

#[test]
fn a_reference_re_read_as_a_binder_is_no_longer_an_occurrence_chirho() {
    // When the parser hands lowering an expression where a pattern belongs, the
    // references in it become binders and constructors, and lose their origins.
    // None of nine ordinary generator, lambda, guard and do forms reach this
    // conversion today (measured 2026-09-22), so it is exercised directly.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let mut stamped_chirho = |text_chirho: &str| {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
        .with_origin_chirho(supply_chirho.fresh_chirho())
    };
    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(stamped_chirho("Just"))),
        arg_chirho: Box::new(ExprChirho::VarChirho(stamped_chirho("y"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let PatChirho::ConChirho {
        con_chirho,
        args_chirho,
        ..
    } = LowerCtxChirho::expr_to_pat_chirho(&expr_chirho)
    else {
        panic!("expected a constructor pattern");
    };
    assert_eq!(con_chirho.origin_chirho(), None);
    let [PatChirho::VarChirho(binder_chirho)] = args_chirho.as_slice() else {
        panic!("expected one variable binder: {args_chirho:?}");
    };
    assert_eq!(binder_chirho.text_chirho(), "y");
    assert_eq!(binder_chirho.origin_chirho(), None);
}
