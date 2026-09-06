// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Given equalities, stuck type-family equalities and the solver behaviours
//! that the rigid-variable lane made load-bearing, end to end through the
//! driver. Verdicts are GHC's.
//! workflow: language-features-chirho/stuck-family-equalities-chirho

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

fn assert_rejected_chirho(file_name_chirho: &str, source_chirho: &str) {
    if check_chirho(file_name_chirho, source_chirho).is_ok() {
        panic!("{file_name_chirho}: GHC rejects this program, we accepted it");
    }
}

#[test]
fn given_family_equality_rewrites_the_family_chirho() {
    assert_accepted_chirho(
        "GivenFamilyChirho.hs",
        "{-# LANGUAGE TypeFamilies #-}\nmodule GivenFamilyChirho where\ntype family F a\ntype instance F Int = Bool\ng :: (F a ~ Bool) => a -> F a -> Bool\ng _ b = not b\n",
    );
}

#[test]
fn given_variable_equality_identifies_rigid_variables_chirho() {
    assert_accepted_chirho(
        "GivenVarEqChirho.hs",
        "{-# LANGUAGE TypeFamilies #-}\nmodule GivenVarEqChirho where\nimport Data.Proxy\ng :: (b ~ c) => Proxy b -> Proxy c\ng x = x\n",
    );
}

#[test]
fn insoluble_given_types_its_unreachable_body_chirho() {
    assert_accepted_chirho(
        "InsolubleGivenChirho.hs",
        "{-# LANGUAGE TypeFamilies #-}\nmodule InsolubleGivenChirho where\nf :: (Int ~ Bool) => Int -> Bool\nf x = f x\nh :: (Int ~ Bool) => Int -> Bool\nh x = x\n",
    );
}

#[test]
fn stuck_family_equality_waits_for_its_argument_chirho() {
    // `n ~ N t` is undecidable until `t` is solved to `TickLabels b n`.
    assert_accepted_chirho(
        "StuckFamilyChirho.hs",
        "{-# LANGUAGE TypeFamilies, ScopedTypeVariables #-}\nmodule StuckFamilyChirho where\ndata TickLabels b n = TickLabels\ntype family N a\ntype instance N (TickLabels b n) = n\nview :: a -> N a\nview = undefined\nrender :: forall n b. TickLabels b n -> n\nrender labels = view labels\n",
    );
}

#[test]
fn associated_family_default_applies_per_instance_chirho() {
    assert_accepted_chirho(
        "AssocDefaultChirho.hs",
        "{-# LANGUAGE TypeFamilies #-}\nmodule AssocDefaultChirho where\nimport Data.Kind (Type)\nclass Cls a where\n    type Fam a :: Type\n    type Fam a = Maybe a\ninstance Cls Int where\n    type Fam Int = Bool\nnott :: (Fam a ~ Bool) => a -> Fam a -> Fam a\nnott _proxy False = True\nnott _proxy True  = False\nfoo :: Bool -> Bool\nfoo = nott (undefined :: Int)\n",
    );
}

#[test]
fn do_block_last_case_meets_the_rigid_result_chirho() {
    assert_accepted_chirho(
        "DoCaseChirho.hs",
        "{-# LANGUAGE TypeFamilies, GADTs #-}\nmodule DoCaseChirho where\ndata S a where\n  S1 :: S Bool\n  S2 :: S Char\ntype family F a where\n  F Bool = Bool\n  F Char = Char\nfoo :: S a -> IO (F a)\nfoo sa = do\n  () <- return ()\n  case sa of\n    S1 -> return False\n    S2 -> return 'x'\n",
    );
}

#[test]
fn type_changing_record_update_chirho() {
    assert_accepted_chirho(
        "RecordUpdateChirho.hs",
        "module RecordUpdateChirho where\ndata Rec a b = Mk { fa :: a, fb :: b }\nupdate :: c -> Rec a b -> Rec c b\nupdate c r = r { fa = c }\n",
    );
    assert_rejected_chirho(
        "RecordUpdateBadChirho.hs",
        "module RecordUpdateBadChirho where\ndata Rec a b = Mk { fa :: a, fb :: b }\nupdate :: c -> Rec a b -> Rec a b\nupdate c r = r { fa = c }\n",
    );
}

#[test]
fn signed_binding_breaks_the_dependency_group_chirho() {
    assert_accepted_chirho(
        "PolyRecGroupChirho.hs",
        "module PolyRecGroupChirho where\ndata Y f = Y (f (Y f))\nmaybeToInt :: Maybe a -> Int\nmaybeToInt = maybe 0 (const 1)\nf :: Y Maybe -> Int\nf (Y x) = g maybeToInt x\ng h x = h $ fmap f x\ntest :: Functor f => (f Int -> b) -> f (Y Maybe) -> b\ntest = g\n",
    );
    // Haskell 98 has no RelaxedPolyRec: `g2` is inferred together with `f2`,
    // monomorphically, and its two uses at different types are an error.
    assert_rejected_chirho(
        "Haskell98GroupChirho.hs",
        "{-# LANGUAGE Haskell98 #-}\nmodule Haskell98GroupChirho where\nf2 :: Eq a => a -> Bool\nf2 x = (x == x) || g2 True || g2 \"Yes\"\ng2 y = (y <= y) || f2 True\n",
    );
}

#[test]
fn functional_dependency_improves_from_givens_and_rigid_instances_chirho() {
    assert_accepted_chirho(
        "FundepGivenChirho.hs",
        "{-# LANGUAGE MultiParamTypeClasses, FunctionalDependencies, FlexibleInstances #-}\nmodule FundepGivenChirho where\nclass MyReader r v | r -> v where\n  myRead :: r -> IO v\ndata R v = R v\ninstance MyReader (R v) v where\n  myRead (R v) = return v\nb :: MyReader r Int => r -> IO ()\nb r = do\n  i <- myRead r\n  if i > 10 then return () else print i\n",
    );
    assert_accepted_chirho(
        "FundepRigidChirho.hs",
        "{-# LANGUAGE MultiParamTypeClasses, FunctionalDependencies, FlexibleInstances #-}\nmodule FundepRigidChirho where\ndata X a = X a\nclass Y a b | a -> b where\n    y :: a -> X b\ninstance Y [[a]] a where\n    y ((x:_):_) = X x\nk :: X a -> X a -> X a\nk _ _ = y ([] ++ [[]] ++ [])\n",
    );
}

#[test]
fn wanted_solved_through_instance_under_givens_chirho() {
    assert_accepted_chirho(
        "InstanceUnderGivensChirho.hs",
        "{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances, ScopedTypeVariables #-}\nmodule InstanceUnderGivensChirho where\nclass Monad m => New a m where\n  new :: m a\ndata T (m :: * -> *) = T\ninstance Monad m => New (T m) m where\n  new = return T\ntest :: forall m. Monad m => m (T m)\ntest = do\n  t <- new\n  return (t :: T m)\n",
    );
}
