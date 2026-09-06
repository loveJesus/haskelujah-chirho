// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rigid type variables (skolems) and GADT refinement, end to end through the
//! driver. Each program's verdict is GHC's: `reject` cases are programs GHC
//! refuses because a signature variable is forced to a concrete type or a
//! constraint cannot be deduced from the signature's context; `accept` cases
//! rely on refinement, scoped type variables, or givens working.
//! workflow: language-features-chirho/rigid-type-variables-chirho

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn check_chirho(file_name_chirho: &str, source_chirho: &str) -> Result<(), String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho)
        .map(|_| ())
        .map_err(|err_chirho| format!("{err_chirho}"))
}

fn assert_accepted_chirho(file_name_chirho: &str, source_chirho: &str) {
    if let Err(err_chirho) = check_chirho(file_name_chirho, source_chirho) {
        panic!("{file_name_chirho}: GHC accepts this program, we reported: {err_chirho}");
    }
}

fn assert_rejected_with_chirho(file_name_chirho: &str, source_chirho: &str, needle_chirho: &str) {
    match check_chirho(file_name_chirho, source_chirho) {
        Ok(()) => panic!("{file_name_chirho}: GHC rejects this program, we accepted it"),
        Err(err_chirho) => assert!(
            err_chirho.contains(needle_chirho),
            "{file_name_chirho}: rejected, but not for the expected reason `{needle_chirho}`: {err_chirho}"
        ),
    }
}

#[test]
fn signature_variable_forced_to_function_type_is_rejected_chirho() {
    assert_rejected_with_chirho(
        "RigidFunChirho.hs",
        "module RigidFunChirho where\nf :: Int -> a\nf x y = x + y\n",
        "type signature mismatch",
    );
}

#[test]
fn signature_result_variable_forced_to_bool_is_rejected_chirho() {
    assert_rejected_with_chirho(
        "RigidResultChirho.hs",
        "module RigidResultChirho where\nf :: a -> a\nf x = True\n",
        "type mismatch",
    );
}

#[test]
fn two_distinct_signature_variables_cannot_be_equated_chirho() {
    assert_rejected_with_chirho(
        "RigidPairChirho.hs",
        "module RigidPairChirho where\nf :: a -> b -> a\nf x y = y\n",
        "type mismatch",
    );
}

#[test]
fn plain_constructor_pattern_on_rigid_variable_is_rejected_chirho() {
    assert_rejected_with_chirho(
        "RigidPatChirho.hs",
        "module RigidPatChirho where\nf :: a -> Int\nf (Just x) = 1\n",
        "type mismatch",
    );
}

#[test]
fn constraint_missing_from_signature_context_is_rejected_chirho() {
    assert_rejected_with_chirho(
        "CouldNotDeduceChirho.hs",
        "module CouldNotDeduceChirho where\nf :: a -> String\nf x = show x\n",
        "could not deduce `Show a`",
    );
}

#[test]
fn local_signature_is_rigid_too_chirho() {
    assert_rejected_with_chirho(
        "LocalRigidChirho.hs",
        "module LocalRigidChirho where\nf :: Int -> Int\nf = g\n  where\n    g :: a -> a\n    g x = x + 1\n",
        "could not deduce `Num a`",
    );
}

#[test]
fn signature_context_discharges_wanteds_chirho() {
    assert_accepted_chirho(
        "GivensChirho.hs",
        "module GivensChirho where\nf :: (Show a, Ord a) => a -> a -> String\nf x y = if x < y then show x else show y\n",
    );
}

#[test]
fn gadt_evaluator_by_equations_refines_per_equation_chirho() {
    assert_accepted_chirho(
        "GadtEvalChirho.hs",
        "{-# LANGUAGE GADTs #-}\nmodule GadtEvalChirho where\ndata Expr a where\n  IntE  :: Int -> Expr Int\n  BoolE :: Bool -> Expr Bool\n  If    :: Expr Bool -> Expr a -> Expr a -> Expr a\n  Pair  :: Expr a -> Expr b -> Expr (a, b)\neval :: Expr a -> a\neval (IntE n) = n\neval (BoolE b) = b\neval (If c t e) = if eval c then eval t else eval e\neval (Pair a b) = (eval a, eval b)\n",
    );
}

#[test]
fn gadt_evaluator_by_case_refines_per_alternative_chirho() {
    assert_accepted_chirho(
        "GadtCaseChirho.hs",
        "{-# LANGUAGE GADTs #-}\nmodule GadtCaseChirho where\ndata Expr a where\n  IntE  :: Int -> Expr Int\n  BoolE :: Bool -> Expr Bool\neval :: Expr a -> a\neval e = case e of\n  IntE n -> n\n  BoolE b -> b\n",
    );
}

#[test]
fn gadt_equality_witness_casts_chirho() {
    assert_accepted_chirho(
        "GadtReflChirho.hs",
        "{-# LANGUAGE GADTs #-}\nmodule GadtReflChirho where\ndata Equal a b where\n  Refl :: Equal a a\ncast :: Equal a b -> a -> b\ncast Refl x = x\n",
    );
}

#[test]
fn gadt_refinement_reaches_guards_and_where_chirho() {
    assert_accepted_chirho(
        "GadtGuardWhereChirho.hs",
        "{-# LANGUAGE GADTs #-}\nmodule GadtGuardWhereChirho where\ndata Expr a where\n  IntE  :: Int -> Expr Int\n  Add   :: Expr Int -> Expr Int -> Expr Int\n  BoolE :: Bool -> Expr Bool\neval :: Expr a -> a\neval (IntE n) | n > 0 = n\n              | otherwise = 0\neval (Add a b) = plus\n  where plus = eval a + eval b\neval (BoolE b) = b\n",
    );
}

#[test]
fn scoped_type_variable_names_the_enclosing_skolem_chirho() {
    assert_accepted_chirho(
        "ScopedChirho.hs",
        "{-# LANGUAGE ScopedTypeVariables #-}\nmodule ScopedChirho where\nf :: forall a. [a] -> [a]\nf xs = go xs\n  where\n    go :: [a] -> [a]\n    go ys = reverse ys ++ xs\n",
    );
}

#[test]
fn rank_n_argument_and_existential_still_accepted_chirho() {
    assert_accepted_chirho(
        "RankNChirho.hs",
        "{-# LANGUAGE RankNTypes #-}\nmodule RankNChirho where\nf :: (forall a. a -> a) -> (Int, Bool)\nf g = (g 1, g True)\n",
    );
    assert_accepted_chirho(
        "ExistentialChirho.hs",
        "{-# LANGUAGE ExistentialQuantification #-}\nmodule ExistentialChirho where\ndata T = forall a. Show a => MkT a\ng :: T -> String\ng (MkT x) = show x\n",
    );
}

#[test]
fn polymorphic_recursion_with_signature_accepted_chirho() {
    assert_accepted_chirho(
        "PolyRecChirho.hs",
        "module PolyRecChirho where\ndata Nested a = Flat a | Nest (Nested [a])\ndepth :: Nested a -> Int\ndepth (Flat _) = 0\ndepth (Nest n) = 1 + depth n\n",
    );
}
