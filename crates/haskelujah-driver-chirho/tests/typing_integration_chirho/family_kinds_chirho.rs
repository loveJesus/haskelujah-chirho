// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family equations must survive into kinds, not merely erase a kind error.
//! The positive and negative sources were checked independently by GHC 9.14.1.

use super::{assert_compile_success_chirho, assert_execution_chirho};
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn constraint_alias_rule_does_not_reject_constraint_families_chirho() {
    // GHC9.14.1 permits this distinct declaration without ConstraintKinds.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, TypeFamilies, KindSignatures, NoConstraintKinds #-}
module FamilyLicenseChirho where
import Data.Kind (Type, Constraint)
type family FamilyChirho aChirho :: Type -> Constraint where
  FamilyChirho Int = Show
"#;
    assert_compile_success_chirho("FamilyLicenseChirho.hs", source_chirho);
}

#[test]
fn constraint_alias_licensing_follows_the_effective_extension_chirho() {
    for extensions_chirho in [
        "Haskell2010",
        "NoConstraintKinds",
        "Haskell2010, ConstraintKinds, NoConstraintKinds",
        "NoConstraintKinds, GHC2021",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE {extensions_chirho} #-}}\nmodule AliasLicenseChirho where\ntype ShowishChirho = Show\n"
        );
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "AliasLicenseChirho.hs",
        )
        .err()
        .expect("GHC-75844 rejects the declaration without ConstraintKinds, even without uses");
        assert!(
            error_chirho.to_string().contains("ConstraintKinds"),
            "{extensions_chirho}: {error_chirho}"
        );
    }
    for extensions_chirho in [
        "GHC2021",
        "Haskell2010, ConstraintKinds",
        "NoConstraintKinds, ConstraintKinds",
        "ConstraintKinds, Haskell2010",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE {extensions_chirho} #-}}\nmodule AliasLicenseChirho where\ntype ShowishChirho = Show\n"
        );
        assert_compile_success_chirho("AliasLicenseChirho.hs", &source_chirho);
    }
    assert_compile_success_chirho(
        "OrdinaryAliasChirho.hs",
        "{-# LANGUAGE Haskell2010 #-}\nmodule OrdinaryAliasChirho where\ntype TypeOnlyChirho = Maybe\n",
    );
}

#[test]
fn local_class_shadows_the_builtin_classifier_chirho() {
    // Eq's unsuffixed spelling is the builtin contract being shadowed here.
    // GHC9.14.1 accepts the same local higher-kinded declaration.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module ShadowClassChirho where
import Prelude hiding (Eq)
import Data.Kind (Type, Constraint)
class Eq (functionChirho :: Type -> Type)
data NeedsHigherChirho (classChirho :: (Type -> Type) -> Constraint)
type LocalChirho = NeedsHigherChirho Eq
"#;
    assert_compile_success_chirho("ShadowClassChirho.hs", source_chirho);
}

const BUILTIN_CLASS_SOURCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, PolyKinds, ConstraintKinds #-}
module Main where
import Data.Kind (Type, Constraint)
type family RekindChirho (classChirho :: leftChirho -> Constraint) :: rightChirho -> Constraint
type instance RekindChirho classChirho = classChirho
type AliasChirho = RekindChirho Eq
data HolderChirho (classChirho :: Type -> Constraint) = HolderChirho Int
valueChirho :: HolderChirho AliasChirho
valueChirho = HolderChirho 42
main :: IO ()
main = case valueChirho of HolderChirho resultChirho -> print resultChirho
"#;

#[test]
fn builtin_class_kind_survives_a_hidden_family_input_chirho() {
    // GHC9.14.1 executes42. The previous kind environment omitted Eq, leaving
    // a fresh classifier outside this alias's otherwise valid binder contract.
    assert_execution_chirho(BUILTIN_CLASS_SOURCE_CHIRHO, "42\n");
}

#[test]
fn first_order_builtin_class_is_not_an_arbitrary_higher_kinded_class_chirho() {
    // GHC-83865: Eq :: Type -> Constraint, not (Type -> Type) -> Constraint.
    let source_chirho = format!(
        "{BUILTIN_CLASS_SOURCE_CHIRHO}\ndata NeedsHigherChirho (classChirho :: (Type -> Type) -> Constraint)\ntype WrongChirho = NeedsHigherChirho Eq\n"
    );
    let error_chirho = typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongBuiltinClassKindChirho.hs",
    )
    .err()
    .expect("a known class kind cannot be invented independently at its use");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn family_equations_match_hidden_kind_inputs_before_visible_arguments_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/FamilyKindSelectionChirho.hs"
    ));
    assert_execution_chirho(source_chirho, "49\n");
    // Disjoint open rows cannot be selected by their registration order.
    let reversed_chirho = source_chirho.replace(
        "type instance PickChirho = Maybe\ntype instance PickChirho = FlagChirho",
        "type instance PickChirho = FlagChirho\ntype instance PickChirho = Maybe",
    );
    assert_execution_chirho(&reversed_chirho, "49\n");
    let implicit_chirho = source_chirho
        .replace("PickChirho @Type Int", "PickChirho Int")
        .replace("PickChirho @Bool 'True", "PickChirho 'True");
    assert_execution_chirho(&implicit_chirho, "49\n");
    let wrong_chirho =
        source_chirho.replace("ordinaryChirho = Just 42", "ordinaryChirho = FlagChirho 42");
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongFamilyKindSelectionChirho.hs",
    )
    .err()
    .expect("the Type equation must not choose the Bool-indexed constructor");
    assert!(
        error_chirho.to_string().contains("type mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn family_equation_rhs_retains_solved_nominal_kind_arguments_chirho() {
    let open_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/OpenFamilyRhsChirho.hs"
    ));
    let closed_chirho = open_chirho.replace(
        "type family BaseChirho valueChirho :: Type -> Type\ntype instance BaseChirho Int = BoxChirho Maybe",
        "type family BaseChirho valueChirho :: Type -> Type where\n  BaseChirho Int = BoxChirho Maybe",
    );
    for source_chirho in [open_chirho, closed_chirho.as_str()] {
        assert_execution_chirho(source_chirho, "True\n");
        let wrong_chirho = source_chirho.replace("BaseChirho Int Bool", "BaseChirho Int Int");
        let error_chirho = typecheck_source_chirho(
            &wrong_chirho,
            &mut SourceMapChirho::new_chirho(),
            "WrongFamilyRhsChirho.hs",
        )
        .err()
        .expect("retaining a nominal kind argument must not erase its ordinary Bool argument");
        let diagnostic_chirho = error_chirho.to_string();
        assert!(
            diagnostic_chirho.contains("Bool") && diagnostic_chirho.contains("Int"),
            "{diagnostic_chirho}"
        );
    }
}

#[test]
fn family_binder_list_kind_is_checked_instead_of_inferred_away_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/ListKindFamilyRhsChirho.hs"
    ));
    assert_compile_success_chirho("ListKindFamilyRhsChirho.hs", source_chirho);
    let wrong_chirho = format!("{source_chirho}\ntype WrongChirho = ResultChirho Maybe Int\n");
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongListKindFamilyRhsChirho.hs",
    )
    .err()
    .expect("Maybe does not take a list-kind argument followed by a value-kind argument");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn recursive_family_rows_keep_the_published_kind_identity_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/RecursiveFamilyKindsChirho.hs"
    ));
    assert_execution_chirho(source_chirho, "49\n");
}

#[test]
fn recursively_constrained_classifiers_finish_and_still_check_arguments_chirho() {
    // The corpus original omits UndecidableInstances and GHC rejects its nested
    // family use. Enabling that extension gives a legal classifier-cycle control.
    let source_chirho = concat!(
        "{-# LANGUAGE UndecidableInstances #-}\n",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../ghc-tests-chirho/typecheck-chirho/should_fail/T15552a.hs"
        )),
    );
    assert_compile_success_chirho("ClassifierCycleChirho.hs", source_chirho);
    let wrong_chirho = format!("{source_chirho}\ntype WrongChirho = GetEntryOfVal 'True\n");
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongClassifierCycleChirho.hs",
    )
    .err()
    .expect("a completed classifier cycle does not permit an argument of kind Bool");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn covering_family_composition_requires_validated_dependencies_chirho() {
    // GHC #13248 rejects the family-headed form wholesale. The independently
    // checked constructor-wrapped form proves the same determining-variable
    // dependency; the unwrapped form exercises our compositional proof.
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/ComposedInjectivityChirho.hs"
    ));
    assert_compile_success_chirho("ComposedInjectivityChirho.hs", source_chirho);
    let wrapped_chirho = source_chirho
        .replace(
            "= WrapChirho (EncodeChirho valueChirho)",
            "= Either () (WrapChirho (EncodeChirho valueChirho))",
        )
        .replace(
            "PipelineChirho Int -> Maybe Int",
            "PipelineChirho Int -> Either () (Maybe Int)",
        );
    assert_compile_success_chirho("WrappedCompositionChirho.hs", &wrapped_chirho);
    let tuple_chirho = source_chirho
        .replace(
            "= WrapChirho (EncodeChirho valueChirho)",
            "= (WrapChirho (EncodeChirho valueChirho), Bool)",
        )
        .replace(
            "PipelineChirho Int -> Maybe Int",
            "PipelineChirho Int -> (Maybe Int, Bool)",
        );
    assert_compile_success_chirho("TupleCompositionChirho.hs", &tuple_chirho);
    for (wrong_chirho, expected_chirho) in [
        (
            source_chirho.replace(
                "WrapChirho (EncodeChirho valueChirho)",
                "WrapChirho (EraseChirho valueChirho)",
            ),
            "injectivity",
        ),
        (
            wrapped_chirho.replace(
                "WrapChirho (EncodeChirho valueChirho)",
                "WrapChirho (EraseChirho valueChirho)",
            ),
            "injectivity",
        ),
        (
            tuple_chirho.replace(
                "WrapChirho (EncodeChirho valueChirho)",
                "WrapChirho (EraseChirho valueChirho)",
            ),
            "injectivity",
        ),
        (
            source_chirho.replace(
                "PipelineChirho Int -> Maybe Int",
                "PipelineChirho Int -> Maybe Bool",
            ),
            "type mismatch",
        ),
    ] {
        let error_chirho = typecheck_source_chirho(
            &wrong_chirho,
            &mut SourceMapChirho::new_chirho(),
            "InvalidCompositionChirho.hs",
        )
        .err()
        .expect("erasing the determining variable or claiming a different result must fail");
        assert!(
            error_chirho.to_string().contains(expected_chirho),
            "{error_chirho}"
        );
    }
}

#[test]
fn indexed_operator_family_rows_keep_their_complete_contract_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, GADTs, PolyKinds, StandaloneKindSignatures, TypeFamilies, TypeOperators #-}
module IndexedOperatorChirho where
import Data.Kind (Type)
data NatChirho = ZeroChirho | SuccChirho NatChirho
type family PlusChirho leftChirho rightChirho where
  PlusChirho ZeroChirho rightChirho = rightChirho
  PlusChirho (SuccChirho leftChirho) rightChirho = SuccChirho (PlusChirho leftChirho rightChirho)
data VecChirho :: Type -> NatChirho -> Type where
  NilChirho :: VecChirho itemChirho ZeroChirho
  ConsChirho :: itemChirho -> VecChirho itemChirho sizeChirho -> VecChirho itemChirho (SuccChirho sizeChirho)
type (+++) :: VecChirho itemChirho leftChirho -> VecChirho itemChirho rightChirho -> VecChirho itemChirho (PlusChirho leftChirho rightChirho)
type family leftChirho +++ rightChirho where
  NilChirho +++ rightChirho = rightChirho
  (ConsChirho itemChirho restChirho) +++ rightChirho = ConsChirho itemChirho (restChirho +++ rightChirho)
"#;
    assert_compile_success_chirho("IndexedOperatorChirho.hs", source_chirho);
}

#[test]
fn inline_family_heads_preserve_their_result_dependency_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies #-}
module InlineFamilyHeadChirho where
import Data.Kind (Type)
data family FamilyChirho (kindChirho :: Type) :: kindChirho
data FlagBoxChirho (flagChirho :: Bool)
type ValueChirho = Maybe (FamilyChirho Type)
type FlagChirho = FlagBoxChirho (FamilyChirho Bool)
"#;
    assert_compile_success_chirho("InlineFamilyHeadChirho.hs", source_chirho);
    let wrong_chirho = source_chirho.replace(
        "FlagBoxChirho (FamilyChirho Bool)",
        "Maybe (FamilyChirho Bool)",
    );
    let error_chirho = typecheck_source_chirho(
        &wrong_chirho,
        &mut SourceMapChirho::new_chirho(),
        "InlineFamilyWrongChirho.hs",
    )
    .err()
    .expect("supplying Bool makes the result kind Bool, not an independently chosen Type");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn remaining_family_binders_cannot_determine_an_outer_result_chirho() {
    let source_chirho = r#"{-# LANGUAGE PolyKinds, RankNTypes, StandaloneKindSignatures, TypeFamilies #-}
module FamilyRemainingBinderChirho where
import Data.Kind (Type)
data family FamilyChirho (kindChirho :: Type) :: kindChirho
type ResultChirho :: forall (outerChirho :: Type) -> forall (innerChirho :: Type) -> outerChirho
type family ResultChirho outerChirho where
  ResultChirho outerChirho = FamilyChirho
"#;
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "FamilyRemainingBinderChirho.hs",
    )
    .err()
    .expect("the RHS depends on its own binder, not the equation's outer argument");
    assert!(
        error_chirho.to_string().contains("family equation result"),
        "{error_chirho}"
    );
    let valid_chirho = source_chirho.replace("-> outerChirho\n", "-> innerChirho\n");
    assert_compile_success_chirho("FamilyMatchingBinderChirho.hs", &valid_chirho);
}

#[test]
fn equation_rows_consume_complete_classifiers_independently_chirho() {
    for (name_chirho, source_chirho) in [
        (
            "FamilyRowClassifiersChirho.hs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/FamilyRowClassifiersChirho.hs"
            )),
        ),
        (
            "FamilyRequiredRowsChirho.hs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/FamilyRequiredRowsChirho.hs"
            )),
        ),
        (
            "FamilyIndexedClassifierChirho.hs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/FamilyIndexedClassifierChirho.hs"
            )),
        ),
    ] {
        assert_compile_success_chirho(name_chirho, source_chirho);
    }
}

#[test]
fn equation_rhs_can_infer_indices_but_cannot_contradict_fixed_ones_chirho() {
    // GHC accepts both inferred-index declarations, including the wildcard
    // Cast RHS. Making those row variables rigid would be a false rejection.
    let inferred_chirho = r#"{-# LANGUAGE PolyKinds, StandaloneKindSignatures, TypeFamilies #-}
module InferredRowChirho where
import Data.Kind (Type)
type PickChirho :: forall kindChirho. kindChirho -> kindChirho
type family PickChirho valueChirho where
  PickChirho valueChirho = Int
"#;
    assert_compile_success_chirho("InferredRowChirho.hs", inferred_chirho);
    let required_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/FamilyRequiredRowsChirho.hs"
    ))
    .replace("= valueChirho", "= Int");
    assert_compile_success_chirho("InferredRequiredRowChirho.hs", &required_chirho);
    let indexed_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/family-equations-chirho/FamilyIndexedClassifierChirho.hs"
    ));
    for source_chirho in [
        required_chirho.replace("CastChirho _ _ Refl", "CastChirho Bool Bool Refl"),
        indexed_chirho.replace(
            "ClassifierChirho OnlyChirho = Type",
            "ClassifierChirho OnlyChirho = Bool",
        ),
    ] {
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "WrongRowChirho.hs",
        )
        .err()
        .expect("the result must have the fixed argument's kind");
        assert!(
            error_chirho.to_string().contains("family equation result"),
            "{error_chirho}"
        );
    }
}

#[test]
fn local_promoted_equality_preserves_its_own_constructor_contract_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, GADTs, PolyKinds, StandaloneKindSignatures, TypeFamilies #-}
module PromotedLocalEqualityChirho where
import Data.Kind (Type)
data RelationChirho (leftChirho :: kindChirho) (rightChirho :: kindChirho) where
  Refl :: RelationChirho valueChirho valueChirho
type CastChirho :: forall aChirho bChirho -> RelationChirho aChirho bChirho -> aChirho -> bChirho
type family CastChirho aChirho bChirho proofChirho valueChirho where
  CastChirho _ _ Refl valueChirho = valueChirho
"#;
    assert_compile_success_chirho("PromotedLocalEqualityChirho.hs", source_chirho);
}

#[test]
fn standalone_family_head_keeps_its_dependent_telescope_chirho() {
    assert_compile_success_chirho(
        "FamilyStandaloneHeadChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyStandaloneHeadChirho.hs"
        )),
    );
}

#[test]
fn standalone_family_result_and_names_are_checked_chirho() {
    for (signature_chirho, expected_chirho) in [
        ("Type -> Bool", "family equation result"),
        ("MissingKindChirho -> Type", "MissingKindChirho"),
        ("forall unusedChirho. valueChirho -> Type", "valueChirho"),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE DataKinds, StandaloneKindSignatures, TypeFamilies #-}}\nmodule FamilyContractChirho where\nimport Data.Kind (Type)\ntype FamilyChirho :: {signature_chirho}\ntype family FamilyChirho valueChirho where\n  FamilyChirho valueChirho = Int\n"
        );
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "FamilyContractChirho.hs",
        )
        .err()
        .expect("a family's written head kind must be checked");
        assert!(
            error_chirho.to_string().contains(expected_chirho),
            "{error_chirho}"
        );
    }
}

#[test]
fn standalone_family_kind_does_not_invent_reduction_arity_or_data_rules_chirho() {
    let source_chirho = r#"{-# LANGUAGE ConstraintKinds, PolyKinds, StandaloneKindSignatures, TypeFamilies #-}
module FamilyPoliciesChirho where
import Data.Kind (Type, Constraint)
type ConstantChirho :: Type -> Type
type family ConstantChirho where
  ConstantChirho = Maybe
valueChirho :: ConstantChirho Int
valueChirho = Nothing
type PredicateChirho :: Type -> Constraint
type family PredicateChirho valueChirho :: Constraint where
  PredicateChirho valueChirho = Show valueChirho
"#;
    assert_compile_success_chirho("FamilyPoliciesChirho.hs", source_chirho);
    let invalid_chirho = source_chirho.replace(
        "type family PredicateChirho valueChirho :: Constraint",
        "type family PredicateChirho valueChirho :: Type -> Type",
    );
    let errors_chirho = typecheck_source_chirho(
        &invalid_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongFamilyPoliciesChirho.hs",
    )
    .err()
    .expect("an inline result cannot contradict the full head kind");
    assert!(
        errors_chirho
            .to_string()
            .contains("type family declaration signature"),
        "{errors_chirho}"
    );
}

#[test]
fn invisible_family_head_binder_is_not_a_visible_equation_argument_chirho() {
    assert_compile_success_chirho(
        "FamilyInvisibleBinderChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyInvisibleBinderChirho.hs"
        )),
    );
}

#[test]
fn family_reduction_can_remove_a_dependent_kind_occurrence_chirho() {
    assert_compile_success_chirho(
        "FamilyDependentConstChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyDependentConstChirho.hs"
        )),
    );
}

#[test]
fn family_equation_variables_do_not_inherit_header_scope_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/FamilyEquationScopeChirho.hs"
    ));
    assert_compile_success_chirho("FamilyEquationScopeChirho.hs", source_chirho);
    let invalid_chirho = source_chirho.replace(
        "SecondChirho ('PairChirho _ aChirho) = aChirho",
        "SecondChirho ('PairChirho _ aChirho) = Int",
    );
    let error_chirho = typecheck_source_chirho(
        &invalid_chirho,
        &mut SourceMapChirho::new_chirho(),
        "FamilyEquationScopeChirho.hs",
    )
    .err()
    .expect("the equation result must still have the declared Bool kind");
    assert!(
        error_chirho.to_string().contains("family equation result"),
        "{error_chirho}"
    );
}

#[test]
fn family_wildcards_infer_their_classifiers_independently_chirho() {
    assert_compile_success_chirho(
        "FamilyWildcardKindsChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyWildcardKindsChirho.hs"
        )),
    );
}

#[test]
fn polymorphic_family_equations_are_not_silently_opaque_chirho() {
    for equation_chirho in [
        "BadChirho (forall bChirho. bChirho -> bChirho) = Int",
        "BadChirho aChirho = forall bChirho. bChirho -> aChirho",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeFamilies, RankNTypes #-}}\nmodule BadFamilyChirho where\ntype family BadChirho aChirho where\n  {equation_chirho}\n"
        );
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "BadFamilyChirho.hs",
        )
        .err()
        .expect("GHC-91510 must reject a polymorphic family equation");
        assert!(
            error_chirho
                .to_string()
                .contains("polymorphic type in family equation"),
            "{error_chirho}"
        );
    }
}

const INJECTIVE_CHIRHO: &str = r#"{-# LANGUAGE DataKinds, GADTs, KindSignatures, PolyKinds, RankNTypes, TypeFamilyDependencies #-}
module InjectiveKindFamilyChirho where
import Data.Kind (Type)
data CodeChirho = CodeIChirho
type family InterpChirho (codeChirho :: CodeChirho) = (resultChirho :: Type) | resultChirho -> codeChirho where
  InterpChirho 'CodeIChirho = Bool
data TermChirho :: forall codeChirho. InterpChirho codeChirho -> Type where
  MkTermChirho :: TermChirho 'False
"#;

#[test]
fn synonym_expansion_precedes_injectivity_validation_chirho() {
    let source_chirho = r#"{-# LANGUAGE TypeFamilyDependencies #-}
module SynonymInjectivityChirho where
type AliasChirho aChirho = Bool
type family FamilyChirho aChirho = resultChirho | resultChirho -> aChirho where
  FamilyChirho aChirho = AliasChirho aChirho
"#;
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "SynonymInjectivityChirho.hs",
    )
    .err()
    .expect("an alias that erases its argument cannot prove injectivity");
    assert!(
        error_chirho.to_string().contains("injectivity loses"),
        "{error_chirho}"
    );
    assert_compile_success_chirho(
        "SynonymInjectivityChirho.hs",
        &source_chirho.replace(
            "type AliasChirho aChirho = Bool",
            "type AliasChirho aChirho = Maybe aChirho",
        ),
    );
}

#[test]
fn closed_family_reduces_inside_a_kind_chirho() {
    let source_chirho = INJECTIVE_CHIRHO
        .replace(
            "= (resultChirho :: Type) | resultChirho -> codeChirho",
            ":: Type",
        )
        .replace(
            "forall codeChirho. InterpChirho codeChirho",
            "InterpChirho 'CodeIChirho",
        );
    assert_compile_success_chirho("ForwardKindFamilyChirho.hs", &source_chirho);
}

#[test]
fn validated_injective_family_improves_its_kind_argument_chirho() {
    assert_compile_success_chirho("InjectiveKindFamilyChirho.hs", INJECTIVE_CHIRHO);
    let source_chirho = INJECTIVE_CHIRHO.replace(
        "= (resultChirho :: Type) | resultChirho -> codeChirho",
        ":: Type",
    );
    assert!(
        typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "NonInjectiveChirho.hs"
        )
        .is_err(),
        "a family without injectivity must not determine an ambiguous argument"
    );
}

#[test]
fn contradictory_injectivity_is_rejected_before_any_use_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, KindSignatures, TypeFamilyDependencies #-}
module InvalidInjectivityChirho where
import Data.Kind (Type)
data CodeChirho = CodeIChirho | CodeJChirho
type family InterpChirho (codeChirho :: CodeChirho) = (resultChirho :: Type) | resultChirho -> codeChirho where
  InterpChirho 'CodeIChirho = Bool
  InterpChirho 'CodeJChirho = Bool
"#;
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "InvalidInjectivityChirho.hs",
    )
    .err()
    .expect("two distinct arguments cannot be recovered from the same result");
    assert!(
        error_chirho.to_string().contains("injectiv"),
        "{error_chirho}"
    );
}
