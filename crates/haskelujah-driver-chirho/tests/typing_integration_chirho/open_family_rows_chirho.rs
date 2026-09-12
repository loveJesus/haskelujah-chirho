// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local open rows are unordered, kind-indexed and owned before their consumers.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{assert_compile_success_chirho, assert_kind_error_chirho};

// GHC9.14.1 hidden-validity reference: an invisible matching argument is
// validated after elaboration. A fixed classifier is not another argument.
const HIDDEN_VALIDITY_PREFIX_CHIRHO: &str = "{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures #-}\nmodule HiddenValidityChirho where\nimport Data.Kind (Type)\n";

#[test]
fn a_stuck_family_in_an_invisible_pattern_is_rejected_chirho() {
    let source_chirho = format!(
        "{HIDDEN_VALIDITY_PREFIX_CHIRHO}type family KChirho (aChirho :: Type) :: Type\ntype family PChirho (aChirho :: kChirho) :: Type\ntype instance PChirho (aChirho :: KChirho Int) = Bool\n"
    );
    let Err(error_chirho) = haskelujah_driver::typecheck_source_chirho(
        &source_chirho,
        &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
        "HiddenFamilyPatternChirho.hs",
    ) else {
        panic!("an invisible family pattern must not be published");
    };
    assert!(
        error_chirho
            .to_string()
            .contains("illegal type family application in an invisible equation pattern"),
        "{error_chirho}"
    );
}

#[test]
fn a_reducible_invisible_classifier_is_valid_chirho() {
    assert_compile_success_chirho(
        "HiddenReducibleFamilyPatternChirho.hs",
        &format!(
            "{HIDDEN_VALIDITY_PREFIX_CHIRHO}type family KChirho (aChirho :: Type) :: Type where\n  KChirho Int = Type\ntype family PChirho (aChirho :: kChirho) :: Type\ntype instance PChirho (aChirho :: KChirho Int) = Bool\n"
        ),
    );
}

#[test]
fn a_fixed_family_classifier_is_not_an_invisible_pattern_chirho() {
    assert_compile_success_chirho(
        "FixedFamilyClassifierChirho.hs",
        &format!(
            "{HIDDEN_VALIDITY_PREFIX_CHIRHO}type family KChirho (aChirho :: Type) :: Type\ntype family PChirho (aChirho :: KChirho Int) :: Type\ntype instance PChirho aChirho = Bool\n"
        ),
    );
}

// Further exact sources independently checked in open-rows-advanced-reference-chirho.jsonl.

#[test]
fn infinite_overlap_with_equal_results_is_rejected_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family InfiniteChirho aChirho bChirho :: Type\ntype instance InfiniteChirho xChirho xChirho = Int\ntype instance InfiniteChirho [yChirho] yChirho = Int\nmain = print (42 :: Int)\n",
    );
}

#[test]
fn same_spelled_row_variables_are_independent_chirho() {
    assert_compile_success_chirho(
        "RowNamespacesChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family NamespacedChirho aChirho bChirho :: Type\ntype instance NamespacedChirho xChirho Int = xChirho\ntype instance NamespacedChirho Bool xChirho = Bool\nmain = print (42 :: Int)\n",
    );
}

#[test]
fn overlap_does_not_unify_right_hand_sides_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family NoRhsChirho aChirho bChirho :: Type\ntype instance NoRhsChirho xChirho yChirho = xChirho\ntype instance NoRhsChirho xChirho yChirho = yChirho\nmain = print (42 :: Int)\n",
    );
}

#[test]
fn hidden_rows_execute_the_reference_witness_on_every_engine_chirho() {
    super::assert_execution_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family PickChirho (aChirho :: kChirho) :: Type\ntype instance PickChirho (aChirho :: Type) = Bool\ntype instance PickChirho (aChirho :: Bool) = Ordering\nwitnessTypeChirho :: PickChirho Int :~: Bool\nwitnessTypeChirho = Refl\nwitnessBoolChirho :: PickChirho 'False :~: Ordering\nwitnessBoolChirho = Refl\nmain = case (witnessTypeChirho, witnessBoolChirho) of (Refl, Refl) -> print (42 :: Int)\n",
        "42\n",
    );
}

#[test]
fn wrong_hidden_row_result_is_rejected_chirho() {
    let error_chirho = haskelujah_driver::typecheck_source_chirho("-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family PickChirho (aChirho :: kChirho) :: Type\ntype instance PickChirho (aChirho :: Type) = Bool\ntype instance PickChirho (aChirho :: Bool) = Ordering\nwitnessTypeChirho :: PickChirho Int :~: Bool\nwitnessTypeChirho = Refl\nwitnessBoolChirho :: PickChirho 'False :~: Bool\nwitnessBoolChirho = Refl\nmain = case (witnessTypeChirho, witnessBoolChirho) of (Refl, Refl) -> print (42 :: Int)\n", &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(), "WrongHiddenWitnessChirho.hs").err().expect("Bool and Ordering cannot both be the result of Pick at Bool");
    assert!(
        error_chirho.to_string().contains("type mismatch"),
        "{error_chirho}"
    );
}
#[test]
fn a_constructor_clash_proves_apartness_even_with_an_infinite_other_input_chirho() {
    assert_compile_success_chirho(
        "CycleThenApartChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family CycleThenApartChirho aChirho bChirho cChirho :: Type\ntype instance CycleThenApartChirho xChirho xChirho Int = Char\ntype instance CycleThenApartChirho [yChirho] yChirho Bool = Ordering\nmain = print (42 :: Int)\n",
    );
}

#[test]
fn matching_family_headed_results_do_not_require_reduction_to_validate_chirho() {
    assert_compile_success_chirho(
        "FamilyRhsAgreementChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications, TypeOperators, GADTs #-}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy(..))\nimport Data.Type.Equality ((:~:)(Refl))\ntype family ResultChirho :: Type\ntype instance ResultChirho = Bool\ntype family OuterChirho aChirho :: Type\ntype instance OuterChirho Int = ResultChirho\ntype instance OuterChirho aChirho = ResultChirho\ntype EvidenceChirho aChirho = Proxy ('False :: OuterChirho aChirho)\nmain = print (42 :: Int)\n",
    );
}

// Exact sources from GHC9.14.1 open-rows-reference-chirho.jsonl.

#[test]
fn open_forward_chirho() {
    assert_compile_success_chirho(
        "OpenForwardChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family GChirho (aChirho :: Type) :: Type\ntype family FChirho (aChirho :: Type) :: GChirho aChirho\ntype instance FChirho Int = True\ntype instance GChirho Int = Bool\n",
    );
}

#[test]
fn open_reverse_chirho() {
    assert_compile_success_chirho(
        "OpenReverseChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype instance GChirho Int = Bool\ntype instance FChirho Int = True\ntype family FChirho (aChirho :: Type) :: GChirho aChirho\ntype family GChirho (aChirho :: Type) :: Type\n",
    );
}

#[test]
fn hidden_inputs_chirho() {
    assert_compile_success_chirho(
        "HiddenInputsChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family PickChirho (aChirho :: kChirho) :: Type\ntype instance PickChirho (aChirho :: Type) = Bool\ntype instance PickChirho (aChirho :: Bool) = Ordering\ntype TypeCaseChirho = Proxy ('False :: PickChirho Int)\ntype BoolCaseChirho = Proxy ('LT :: PickChirho 'False)\n",
    );
}

#[test]
fn explicit_hidden_inputs_chirho() {
    assert_compile_success_chirho(
        "ExplicitHiddenInputsChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family PickChirho (aChirho :: kChirho) :: Type\ntype instance PickChirho (aChirho :: Type) = Bool\ntype instance PickChirho (aChirho :: Bool) = Ordering\ntype TypeCaseChirho = Proxy ('False :: PickChirho @Type Int)\ntype BoolCaseChirho = Proxy ('LT :: PickChirho @Bool 'False)\n",
    );
}

#[test]
fn compatible_overlap_chirho() {
    assert_compile_success_chirho(
        "CompatibleOverlapChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family SameChirho aChirho :: Type\ntype instance SameChirho Int = Bool\ntype instance SameChirho aChirho = Bool\ntype EvidenceChirho aChirho = Proxy ('False :: SameChirho aChirho)\n",
    );
}

#[test]
fn reversed_overlap_chirho() {
    assert_compile_success_chirho(
        "ReversedOverlapChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family SameChirho aChirho :: Type\ntype instance SameChirho aChirho = Bool\ntype instance SameChirho Int = Bool\ntype EvidenceChirho aChirho = Proxy ('False :: SameChirho aChirho)\n",
    );
}

#[test]
fn conflicting_overlap_is_rejected_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family SameChirho aChirho :: Type\ntype instance SameChirho Int = Char\ntype instance SameChirho aChirho = Bool\n",
    );
}

#[test]
fn infinite_overlap_is_rejected_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family LoopChirho aChirho bChirho :: Type\ntype instance LoopChirho xChirho xChirho = Int\ntype instance LoopChirho [yChirho] yChirho = Bool\n",
    );
}

#[test]
fn unbound_result_is_rejected_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, TypeApplications #-}\nmodule OpenRowsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family MissingChirho aChirho :: Type\ntype instance MissingChirho Int = unknownChirho\n",
    );
}
