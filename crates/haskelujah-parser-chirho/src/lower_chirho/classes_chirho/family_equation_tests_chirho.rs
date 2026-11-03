// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: closed type-family equations keep their written head, their
//! arguments in order, their explicit binders and the binders left unused (row 686).
//! The shapes are the row 686 oracle programs GHC 9.14.1 measured.

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TypeFamilyEquationChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::test_support_chirho::parse_and_lower_chirho;

/// The equations of the closed family `family_chirho` in `source_chirho`.
fn equations_of_chirho(source_chirho: &str, family_chirho: &str) -> Vec<TypeFamilyEquationChirho> {
    parse_and_lower_chirho(source_chirho)
        .decls_chirho
        .into_iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                equations_chirho,
                ..
            } if name_chirho.text_chirho() == family_chirho => Some(equations_chirho),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no closed family `{family_chirho}` in the module"))
}

/// A compact spelling of an argument pattern, enough to tell shapes apart.
fn render_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) | TypeChirho::VarChirho(name_chirho) => {
            name_chirho.full_name_chirho()
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} {})",
            render_chirho(fun_chirho),
            render_chirho(arg_chirho)
        ),
        TypeChirho::ParenChirho { inner_chirho, .. } => render_chirho(inner_chirho),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", render_chirho(element_chirho))
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            format!("'{}", name_chirho.text_chirho())
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => format!(
            "'[{}]",
            elements_chirho
                .iter()
                .map(render_chirho)
                .collect::<Vec<_>>()
                .join(",")
        ),
        TypeChirho::WildcardChirho { .. } => "_".to_string(),
        TypeChirho::LitChirho { value_chirho, .. } => value_chirho.clone(),
        other_chirho => format!("{other_chirho:?}"),
    }
}

/// `head arg …` for one equation, or `<no head>`.
fn shape_chirho(equation_chirho: &TypeFamilyEquationChirho) -> String {
    let head_chirho = equation_chirho.head_chirho.as_ref().map_or_else(
        || "<no head>".to_string(),
        |head_chirho| head_chirho.full_name_chirho(),
    );
    let arguments_chirho: Vec<String> = equation_chirho
        .lhs_types_chirho
        .iter()
        .map(render_chirho)
        .collect();
    std::iter::once(head_chirho)
        .chain(arguments_chirho)
        .collect::<Vec<_>>()
        .join(" ")
}

fn shapes_chirho(source_chirho: &str, family_chirho: &str) -> Vec<String> {
    equations_of_chirho(source_chirho, family_chirho)
        .iter()
        .map(shape_chirho)
        .collect()
}

const FORMS_SOURCE_CHIRHO: &str =
    "{-# LANGUAGE DataKinds, TypeFamilies, TypeOperators, PolyKinds, KindSignatures #-}
module Main where

data NatChirho = ZeroChirho | SuccChirho NatChirho

type family (aChirho :: NatChirho) + (bChirho :: NatChirho) :: NatChirho where
  ZeroChirho + bChirho = bChirho
  SuccChirho aChirho + bChirho = SuccChirho (aChirho + bChirho)
infixl 6 +

type family (aChirho :: NatChirho) ** (bChirho :: NatChirho) :: NatChirho where
  (**) ZeroChirho bChirho = ZeroChirho
  (**) (SuccChirho aChirho) bChirho = SuccChirho (SuccChirho (aChirho ** bChirho))
infixl 7 **

type family PlusChirho (aChirho :: NatChirho) (bChirho :: NatChirho) :: NatChirho where
  ZeroChirho `PlusChirho` bChirho = bChirho
  SuccChirho aChirho `PlusChirho` bChirho = SuccChirho (aChirho `PlusChirho` bChirho)

type family (aChirho :: NatChirho) `MinusChirho` (bChirho :: NatChirho) :: NatChirho where
  MinusChirho aChirho ZeroChirho = aChirho
  MinusChirho (SuccChirho aChirho) (SuccChirho bChirho) = MinusChirho aChirho bChirho
";

#[test]
fn every_written_head_form_keeps_its_arguments_in_order_chirho() {
    assert_eq!(
        shapes_chirho(FORMS_SOURCE_CHIRHO, "+"),
        ["+ ZeroChirho bChirho", "+ (SuccChirho aChirho) bChirho"],
        "infix heads take the fixity declared after the family"
    );
    assert_eq!(
        shapes_chirho(FORMS_SOURCE_CHIRHO, "**"),
        ["** ZeroChirho bChirho", "** (SuccChirho aChirho) bChirho"]
    );
    assert_eq!(
        shapes_chirho(FORMS_SOURCE_CHIRHO, "PlusChirho"),
        [
            "PlusChirho ZeroChirho bChirho",
            "PlusChirho (SuccChirho aChirho) bChirho"
        ]
    );
    assert_eq!(
        shapes_chirho(FORMS_SOURCE_CHIRHO, "MinusChirho"),
        [
            "MinusChirho aChirho ZeroChirho",
            "MinusChirho (SuccChirho aChirho) (SuccChirho bChirho)"
        ]
    );
}

#[test]
fn split_wd_equations_are_headed_by_their_operator_chirho() {
    let source_chirho = "{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeOperators, GADTs #-}
module SplitWD where
import Data.Kind ( Type )
data Nat = Zero | Succ Nat
type family a + b where
  Zero + b = b
  Succ a + b = Succ (a + b)
infixl 6 +
data Vec :: Type -> Nat -> Type where
  VNil :: Vec a Zero
  (:>) :: a -> Vec a n -> Vec a (Succ n)
infixr 5 :>
type (+++) :: Vec a n -> Vec a m -> Vec a (n + m)
type family xs +++ ys where
  VNil +++ ys = ys
  (x :> xs) +++ ys = x :> (xs +++ ys)
infixr 5 +++
";
    assert_eq!(
        shapes_chirho(source_chirho, "+"),
        ["+ Zero b", "+ (Succ a) b"]
    );
    assert_eq!(
        shapes_chirho(source_chirho, "+++"),
        ["+++ VNil ys", "+++ ((:> x) xs) ys"]
    );
}

#[test]
fn parenthesized_and_self_qualified_heads_are_heads_chirho() {
    let parenthesized_chirho = "{-# LANGUAGE TypeFamilies #-}
module Main where
type family TwoChirho aChirho bChirho where
  (TwoChirho aChirho) bChirho = (aChirho, bChirho)
";
    assert_eq!(
        shapes_chirho(parenthesized_chirho, "TwoChirho"),
        ["TwoChirho aChirho bChirho"],
        "GHC 9.14.1 accepts a parenthesized partial head (oracle A07)"
    );
    let qualified_chirho = "{-# LANGUAGE TypeFamilies #-}
module R08QualifiedHeadChirho where
type family FChirho aChirho where
  R08QualifiedHeadChirho.FChirho Int = Bool
";
    assert_eq!(
        shapes_chirho(qualified_chirho, "FChirho"),
        ["R08QualifiedHeadChirho.FChirho Int"],
        "the head keeps its own-module qualifier for validity to judge (oracle R08)"
    );
}

#[test]
fn explicit_binders_keep_kinds_and_only_unused_ones_are_unwritten_chirho() {
    let source_chirho =
        "{-# LANGUAGE DataKinds, TypeFamilies, PolyKinds, KindSignatures, ExplicitForAll #-}
module Main where
type family GChirho where
  forall bChirho. GChirho = ()
type family KindUseChirho (aChirho :: kChirho) :: Bool where
  forall kChirho (aChirho :: kChirho). KindUseChirho aChirho = True
type family ElemChirho aChirho where
  forall xChirho. ElemChirho [xChirho] = xChirho
type family KindPatternChirho (aChirho :: kChirho) :: Bool where
  forall kChirho aChirho. KindPatternChirho (aChirho :: kChirho) = True
type family ShadowChirho aChirho where
  forall aChirho. ShadowChirho (forall aChirho. aChirho) = Int
type family RightOnlyChirho aChirho where
  forall aChirho. RightOnlyChirho Int = aChirho
";
    let unwritten_chirho = |family_chirho: &str| -> Vec<String> {
        equations_of_chirho(source_chirho, family_chirho)[0]
            .unwritten_binders_chirho
            .iter()
            .map(|name_chirho| name_chirho.text_chirho().to_string())
            .collect()
    };
    assert_eq!(unwritten_chirho("GChirho"), ["bChirho"], "oracle R09/R10");
    assert!(
        unwritten_chirho("KindUseChirho").is_empty(),
        "a binder used only in a used binder's kind is used (oracle A08)"
    );
    assert!(unwritten_chirho("ElemChirho").is_empty());
    assert!(
        unwritten_chirho("KindPatternChirho").is_empty(),
        "a kind annotation in a pattern is a use even though lowering drops it"
    );
    assert_eq!(
        unwritten_chirho("ShadowChirho"),
        ["aChirho"],
        "an inner forall that rebinds the name hides it"
    );
    assert_eq!(
        unwritten_chirho("RightOnlyChirho"),
        ["aChirho"],
        "the right side never counts"
    );

    let kind_use_chirho = &equations_of_chirho(source_chirho, "KindUseChirho")[0];
    let binders_chirho = kind_use_chirho
        .explicit_binders_chirho
        .as_ref()
        .expect("the written forall is kept");
    let kinds_chirho: Vec<(String, bool)> = binders_chirho
        .iter()
        .map(|binder_chirho| {
            (
                binder_chirho.name_chirho.text_chirho().to_string(),
                binder_chirho.kind_annotation_chirho.is_some(),
            )
        })
        .collect();
    assert_eq!(
        kinds_chirho,
        [
            ("kChirho".to_string(), false),
            ("aChirho".to_string(), true)
        ],
        "full TyVar binders, kind annotations included"
    );
    assert_eq!(shape_chirho(kind_use_chirho), "KindUseChirho aChirho");
    assert!(
        equations_of_chirho(source_chirho, "ElemChirho")[0]
            .explicit_binders_chirho
            .is_some()
    );
    let no_forall_chirho = &equations_of_chirho(FORMS_SOURCE_CHIRHO, "+")[0];
    assert!(no_forall_chirho.explicit_binders_chirho.is_none());
    assert!(no_forall_chirho.unwritten_binders_chirho.is_empty());
}

#[test]
fn each_equation_has_its_own_span_chirho() {
    let equations_chirho = equations_of_chirho(FORMS_SOURCE_CHIRHO, "+");
    let spans_chirho: Vec<(u32, u32)> = equations_chirho
        .iter()
        .map(|equation_chirho| {
            (
                equation_chirho.span_chirho.start_chirho().as_u32_chirho(),
                equation_chirho.span_chirho.end_chirho().as_u32_chirho(),
            )
        })
        .collect();
    let first_chirho = FORMS_SOURCE_CHIRHO
        .find("ZeroChirho + bChirho = bChirho")
        .expect("first equation") as u32;
    let second_chirho = FORMS_SOURCE_CHIRHO
        .find("SuccChirho aChirho + bChirho")
        .expect("second equation") as u32;
    assert_eq!(spans_chirho[0].0, first_chirho);
    assert_eq!(spans_chirho[1].0, second_chirho);
    assert!(
        spans_chirho[0].1 <= second_chirho,
        "spans do not overlap: {spans_chirho:?}"
    );
}

#[test]
fn a_left_side_continued_on_the_next_line_is_one_equation_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}
module Main where
type family PairChirho aChirho bChirho where
  PairChirho aChirho
    bChirho = (aChirho, bChirho)
main :: IO ()
main = print ((1, 'c') :: PairChirho Int Char)
";
    let equations_chirho = equations_of_chirho(source_chirho, "PairChirho");
    assert_eq!(
        equations_chirho.len(),
        1,
        "oracle A06's continued left side"
    );
    assert_eq!(
        shape_chirho(&equations_chirho[0]),
        "PairChirho aChirho bChirho"
    );
    assert!(!equations_chirho[0].malformed_chirho);
}

#[test]
fn incomplete_and_headless_equations_are_kept_and_marked_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies, DataKinds #-}
module Main where
type family MissingChirho aChirho where
  MissingChirho Int
  MissingChirho Bool = Char
type family ContextChirho aChirho where
  Show aChirho => ContextChirho aChirho = Int
type family VariableHeadChirho aChirho where
  aChirho Int = Bool
type family LiteralHeadChirho aChirho where
  3 = Int
type family LeftoverChirho aChirho where
  LeftoverChirho Int :: Bool = Char
";
    let missing_chirho = equations_of_chirho(source_chirho, "MissingChirho");
    assert_eq!(
        missing_chirho.len(),
        2,
        "the incomplete equation is kept, not dropped"
    );
    assert!(missing_chirho[0].malformed_chirho, "no `=` (oracle R06)");
    assert!(!missing_chirho[1].malformed_chirho);
    assert_eq!(shape_chirho(&missing_chirho[1]), "MissingChirho Bool");
    assert!(
        equations_of_chirho(source_chirho, "ContextChirho")[0].malformed_chirho,
        "a context (oracle R07)"
    );
    for family_chirho in ["VariableHeadChirho", "LiteralHeadChirho"] {
        let equation_chirho = &equations_of_chirho(source_chirho, family_chirho)[0];
        assert!(
            equation_chirho.head_chirho.is_none(),
            "{family_chirho} (oracles R04/R05)"
        );
        assert!(equation_chirho.is_held_chirho());
    }
    assert!(
        equations_of_chirho(source_chirho, "LeftoverChirho")[0].malformed_chirho,
        "tokens the left-side type grammar leaves before `=`"
    );
}
