// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

#[path = "typing_integration_chirho/kind_terms_chirho.rs"]
mod kind_terms_chirho;

#[test]
fn promoted_opaque_occurrences_keep_independent_kinds_and_check_the_body_chirho() {
    // GHC 9.14.1 accepts the source and rejects the Int-result mutation (GHC-83865).
    // This pins occurrence independence, not complete promoted existential metadata.
    let source_chirho = r#"{-# LANGUAGE ExistentialQuantification, PolyKinds, DataKinds, RankNTypes, GADTs, TypeOperators #-}
module PromotedChirho where
import Data.Kind (Type)
import Data.Type.Equality
data WrappedChirho = forall aChirho. WrapChirho aChirho
matchChirho :: forall kaChirho kbChirho (aChirho :: kaChirho) (bChirho :: kbChirho).
  ('WrapChirho aChirho :~: 'WrapChirho bChirho) -> Bool
matchChirho Refl = True
"#;
    assert_compile_success_chirho("PromotedChirho.hs", source_chirho);
    let invalid_chirho = source_chirho.replace("-> Bool", "-> Int");
    let error_chirho = haskelujah_driver::typecheck_source_chirho(
        &invalid_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongPromotedChirho.hs",
    )
    .err()
    .expect("the result still must match its signature");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| diagnostic_chirho.code_chirho
                == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(200))),
        "{error_chirho}"
    );
}

const FAMILY_PATTERNS_SOURCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeFamilies, TypeOperators #-}
module Main where
import Data.Kind (Type)
import GHC.TypeLits (Nat, Symbol)
type family SelectChirho (xsChirho :: [Type]) (rChirho :: Type) where
  SelectChirho '[] rChirho = rChirho
  SelectChirho (aChirho ': asChirho) rChirho = Bool
emptyChirho :: SelectChirho '[] Int
emptyChirho = 7
nonemptyChirho :: SelectChirho '[Char] Int
nonemptyChirho = True
type family UnwrapChirho aChirho where
  UnwrapChirho (Maybe elementChirho) = elementChirho
unwrapChirho :: UnwrapChirho (Maybe Int) -> Int
unwrapChirho valueChirho = valueChirho
type family NestedChirho (xsChirho :: [[Type]]) rChirho
type instance NestedChirho '[ '[firstChirho], '[secondChirho]] (Maybe rChirho) = (firstChirho, secondChirho)
nestedChirho :: NestedChirho '[ '[Int], '[Bool]] (Maybe Char)
nestedChirho = (13, True)
type family HigherChirho (xsChirho :: [Type]) :: Type -> Type where
  HigherChirho '[] = Maybe
  HigherChirho (aChirho ': asChirho) = []
higherChirho :: HigherChirho '[] Int
higherChirho = Just 17
type family LiteralChirho (nChirho :: Nat) (sChirho :: Symbol) where
  LiteralChirho 0 "zero" = Int
  LiteralChirho 1 "one" = Bool
zeroChirho :: LiteralChirho 0 "zero"
zeroChirho = 19
oneChirho :: LiteralChirho 1 "one"
oneChirho = True
type family aChirho :*: bChirho where
  Int :*: Bool = Char
infixChirho :: Int :*: Bool
infixChirho = 'c'
main :: IO ()
main = do
  print emptyChirho
  print nonemptyChirho
  print (unwrapChirho 11)
  print nestedChirho
  print higherChirho
  print zeroChirho
  print oneChirho
  print infixChirho
"#;

#[test]
fn family_patterns_select_the_ghc_result_on_every_engine_chirho() {
    // Unchanged source independently executed by GHC 9.14.1. Each result
    // depends on a recovered pattern, equation-local variable or extra argument.
    assert_execution_chirho(
        FAMILY_PATTERNS_SOURCE_CHIRHO,
        "7\nTrue\n11\n(13,True)\nJust 17\n19\nTrue\n'c'\n",
    );
}

#[test]
fn family_pattern_selection_rejects_wrong_result_types_chirho() {
    // GHC-83865 for both single-expression mutations: Bool/Char and Int/Bool.
    for (from_chirho, to_chirho) in [
        ("nonemptyChirho = True", "nonemptyChirho = 'x'"),
        ("zeroChirho = 19", "zeroChirho = True"),
    ] {
        let source_chirho = FAMILY_PATTERNS_SOURCE_CHIRHO.replace(from_chirho, to_chirho);
        let error_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "WrongFamilyResultChirho.hs",
        )
        .map(|_| ())
        .expect_err("the selected equation must constrain the result");
        assert!(
            error_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diagnostic_chirho| {
                    diagnostic_chirho.code_chirho
                        == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(200))
                }),
            "{error_chirho}"
        );
    }
}

// Independently executed under GHC 9.14.1: 42 / 7 / 11 / True.
// The infix and context orders differ from normalized body traversal; the
// explicit forall and prefix form are controls that must retain their order.
const SOURCE_ORDER_CHIRHO: &str = r#"{-# LANGUAGE TypeOperators, TypeApplications, ExplicitForAll #-}
module Main where
implicitChirho :: aChirho `opChirho` bChirho -> aChirho `opChirho` bChirho
implicitChirho valueChirho = valueChirho
explicitChirho :: forall opChirho aChirho bChirho. aChirho `opChirho` bChirho -> aChirho `opChirho` bChirho
explicitChirho valueChirho = valueChirho
prefixChirho :: opChirho aChirho bChirho -> opChirho aChirho bChirho
prefixChirho valueChirho = valueChirho
contextChirho :: (Eq bChirho, Eq aChirho) => aChirho -> bChirho -> Bool
contextChirho leftChirho rightChirho = leftChirho == leftChirho && rightChirho == rightChirho
main :: IO ()
main = do
  print (fst (implicitChirho @Int @(,) @Bool (42, True)))
  print (snd (explicitChirho @(,) @Bool @Int (False, 7)))
  print (fst (prefixChirho @(,) @Int @Bool (11, True)))
  print (contextChirho @Bool @Int 9 False)
"#;

fn assert_compile_success_chirho(file_name_chirho: &str, source_chirho: &str) {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho);
    assert!(
        result_chirho.is_ok(),
        "expected compile_source_chirho to succeed for {file_name_chirho}, got {:?}",
        result_chirho.err()
    );
}

fn assert_kind_error_chirho(source_chirho: &str) {
    let error_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "InvalidKindContractChirho.hs",
    )
    .map(|_| ())
    .expect_err("the reference-invalid kind contract must be diagnosed");
    let message_chirho = error_chirho.to_string();
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho.code_chirho
                    == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300))
            }),
        "{message_chirho}"
    );
}

#[test]
fn grouped_family_parameters_and_implicit_predicates_keep_their_arity_chirho() {
    // GHC 9.14.1 accepts all four family arguments and rejects the missing
    // fourth with GHC-83865. A predicate used before its first ordinary type
    // occurrence must still constrain Dict's parameter to Constraint.
    let prefix_chirho = r#"{-# LANGUAGE PolyKinds, DataKinds, TypeFamilies, MultiParamTypeClasses, ConstraintKinds, GADTs #-}
module HeadContextChirho where
import Data.Kind (Type, Constraint)
type family CurryChirho (fChirho :: Type -> Type) (xsChirho :: [Type]) (rChirho :: Type) (aChirho :: Type) :: Constraint
class AllChirho (cChirho :: kChirho -> Constraint) (xsChirho :: [kChirho])
data DictChirho cChirho where
  MkDictChirho :: cChirho => DictChirho cChirho
"#;
    assert_compile_success_chirho(
        "HeadContextChirho.hs",
        &format!(
            "{prefix_chirho}familyWitnessChirho :: DictChirho (CurryChirho Maybe '[Int] Bool Char)\nfamilyWitnessChirho = undefined\nclassWitnessChirho :: DictChirho (AllChirho Eq '[Int])\nclassWitnessChirho = undefined\n"
        ),
    );
    assert_kind_error_chirho(&format!(
        "{prefix_chirho}familyWitnessChirho :: DictChirho (CurryChirho Maybe '[Int] Bool)\nfamilyWitnessChirho = undefined\n"
    ));
}

#[test]
fn signature_elaboration_metavariables_are_not_written_kind_contracts_chirho() {
    // Both accepted independently by GHC 9.14.1. Elaborating an indexed kind
    // can solve fresh application-result metas; they are not source variables
    // whose later specialization by a constructor body must be forbidden.
    assert_compile_success_chirho(
        "IndexedTailChirho.hs",
        r#"{-# LANGUAGE PolyKinds, DataKinds, GADTs, NoCUSKs #-}
module IndexedTailChirho where
import Data.Kind (Type)
data DChirho :: Type -> Type
data SDChirho :: forall aChirho. DChirho aChirho -> Type
"#,
    );
    assert_compile_success_chirho(
        "ConcreteRuntimeTailChirho.hs",
        r#"{-# LANGUAGE PolyKinds, DataKinds, GADTs, TypeFamilies, MagicHash, UnliftedNewtypes #-}
module ConcreteRuntimeTailChirho where
import GHC.Exts
data ColorChirho = RedChirho
type family InterpretChirho (xChirho :: ColorChirho) :: RuntimeRep where
  InterpretChirho 'RedChirho = 'IntRep
newtype QuuxChirho :: TYPE (InterpretChirho RedChirho) where
  MkQChirho :: Int# -> QuuxChirho
"#,
    );
}

#[test]
fn written_kind_contracts_are_rigid_but_unannotated_heads_infer_chirho() {
    // Each variant independently checked with GHC 9.14.1, including the
    // alpha-renamed binder: a standalone signature does not scope its names
    // over the declaration, but its quantified kind cannot be specialized.
    let prefix_chirho = "{-# LANGUAGE PolyKinds, StandaloneKindSignatures #-}\nmodule KindContractChirho where\nimport Data.Kind (Type)\ntype BoxChirho :: forall kChirho. kChirho -> Type\n";
    assert_kind_error_chirho(&format!(
        "{prefix_chirho}data BoxChirho (aChirho :: Type)\n"
    ));
    for declaration_chirho in [
        "data BoxChirho aChirho = MkBoxChirho",
        "data BoxChirho (aChirho :: jChirho) = MkBoxChirho",
    ] {
        assert_compile_success_chirho(
            "KindContractChirho.hs",
            &format!("{prefix_chirho}{declaration_chirho}\n"),
        );
    }
}

#[test]
fn rigid_kinds_are_checked_in_ordinary_and_explicit_constructor_fields_chirho() {
    // Both complete sources are GHC-25897, not unknown-name controls.
    for declaration_chirho in [
        "data BoxChirho (aChirho :: kChirho) = MkBoxChirho aChirho",
        "data BoxChirho aChirho where\n  MkBoxChirho :: forall kChirho (bChirho :: kChirho). bChirho -> BoxChirho bChirho",
    ] {
        assert_kind_error_chirho(&format!(
            "{{-# LANGUAGE GADTs, PolyKinds, ExplicitForAll, NoCUSKs #-}}\nmodule RigidFieldsChirho where\n{declaration_chirho}\n"
        ));
    }
    assert_compile_success_chirho(
        "InferredFieldChirho.hs",
        "{-# LANGUAGE PolyKinds, NoCUSKs #-}\nmodule InferredFieldChirho where\ndata BoxChirho aChirho = MkBoxChirho (Maybe aChirho)\n",
    );
}

#[test]
fn cusk_and_incomplete_recursion_use_different_kind_bindings_chirho() {
    // Identical body: independently accepted with CUSKs and rejected with
    // NoCUSKs by GHC 9.14.1. The declaration, not its spelling, owns recursion.
    let program_chirho = |extension_chirho: &str| {
        format!(
            "{{-# LANGUAGE PolyKinds, GADTs, {extension_chirho} #-}}\nmodule KindRecursionChirho where\nimport Data.Kind (Type)\ndata TChirho (mChirho :: kChirho -> Type) :: kChirho -> Type where\n  MkTChirho :: mChirho aChirho -> TChirho Maybe (mChirho aChirho) -> TChirho mChirho aChirho\n"
        )
    };
    assert_compile_success_chirho("CompleteKindRecursionChirho.hs", &program_chirho("CUSKs"));
    assert_kind_error_chirho(&program_chirho("NoCUSKs"));
    assert_compile_success_chirho(
        "OrdinaryMonoChirho.hs",
        "{-# LANGUAGE PolyKinds, NoCUSKs #-}\nmodule OrdinaryMonoChirho where\ndata TChirho mChirho aChirho = MkTChirho (mChirho aChirho) (TChirho Maybe (mChirho aChirho))\n",
    );
}

#[test]
fn kind_inference_uses_local_dependencies_before_generalizing_chirho() {
    // The forward forms of these independent modules were accepted by GHC
    // 9.14.1. Neither a later local type nor a recursive peer is an import.
    for (prefix_chirho, first_chirho, second_chirho) in [
        (
            "{-# LANGUAGE KindSignatures #-}\nmodule ForwardClassChirho where\nimport Data.Kind (Type)\n",
            "class SumSizeChirho fChirho where\n  sumSizeChirho :: TaggedChirho fChirho\n",
            "newtype TaggedChirho (sChirho :: Type -> Type) = TaggedChirho { unTaggedChirho :: Int }\n",
        ),
        (
            "{-# LANGUAGE PolyKinds, NoCUSKs #-}\nmodule ForwardPolykindChirho where\nimport Data.Proxy (Proxy)\n",
            "data UsesChirho = UsesChirho (BoxChirho Int) (BoxChirho Maybe)\n",
            "data BoxChirho aChirho = BoxChirho (Proxy aChirho)\n",
        ),
        (
            "{-# LANGUAGE PolyKinds, NoCUSKs #-}\nmodule RecursiveKindsChirho where\n",
            "data LeftChirho aChirho = LeftChirho (RightChirho aChirho) (aChirho Int)\n",
            "data RightChirho bChirho = RightChirho (LeftChirho bChirho)\nuseRightChirho :: RightChirho Maybe -> RightChirho Maybe\nuseRightChirho valueChirho = valueChirho\n",
        ),
    ] {
        for declarations_chirho in [
            format!("{first_chirho}{second_chirho}"),
            format!("{second_chirho}{first_chirho}"),
        ] {
            assert_compile_success_chirho(
                "KindDependenciesChirho.hs",
                &format!("{prefix_chirho}{declarations_chirho}"),
            );
        }
    }
}

#[test]
fn complete_kind_signatures_break_inference_cycles_without_skipping_bodies_chirho() {
    // GHC 9.14.1 accepts the signed cycle and rejects the unsigned cycle.
    // Checking both bodies in one inference SCC would constrain Unsigned at
    // Int before it was generalized and wrongly reject the Maybe occurrence.
    let definitions_chirho = "data SignedChirho aChirho = MkSignedChirho (UnsignedChirho Int) (UnsignedChirho Maybe)\ndata UnsignedChirho bChirho = MkUnsignedChirho (SignedChirho bChirho)\n";
    let prefix_chirho = "{-# LANGUAGE PolyKinds, StandaloneKindSignatures #-}\nmodule SignedRecursionChirho where\nimport Data.Kind (Type)\n";
    assert_compile_success_chirho(
        "SignedRecursionChirho.hs",
        &format!(
            "{prefix_chirho}type SignedChirho :: forall kChirho. kChirho -> Type\n{definitions_chirho}"
        ),
    );
    assert_kind_error_chirho(&format!("{prefix_chirho}{definitions_chirho}"));
}

#[test]
fn superclass_kinds_reach_the_local_class_before_publication_chirho() {
    // The unchanged sources were independently checked with GHC 9.14.1.
    assert_compile_success_chirho(
        "MonadSuperclassChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010 #-}\nmodule MonadSuperclassChirho where\nclass Monad m_chirho => CChirho m_chirho\ninstance CChirho Maybe\n",
    );
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010 #-}\nmodule BadMonadSuperclassChirho where\nclass Monad m_chirho => CChirho m_chirho\ninstance CChirho Int\n",
    );
    assert_compile_success_chirho(
        "VariablePredicateChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010, ConstraintKinds, QuantifiedConstraints, UndecidableSuperClasses, MultiParamTypeClasses, TypeOperators, GADTs #-}\nmodule VariablePredicateChirho where\nclass CChirho a_chirho b_chirho\nclass (forall x_chirho. f_chirho x_chirho) => LimitChirho f_chirho\ndata DChirho c_chirho where\n  DChirho :: c_chirho => DChirho c_chirho\nwitnessChirho :: DChirho (LimitChirho (CChirho Int))\nwitnessChirho = undefined\n",
    );
}

#[test]
fn recursive_written_kinds_may_equate_but_not_specialize_chirho() {
    assert_compile_success_chirho(
        "MutualWrittenKindsChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, GADTs #-}\nmodule MutualWrittenKindsChirho where\ndata TChirho (a_chirho :: k1_chirho) k2_chirho (x_chirho :: k2_chirho) = MkTChirho (SChirho a_chirho k2_chirho x_chirho)\ndata SChirho (b_chirho :: k3_chirho) k4_chirho (y_chirho :: k4_chirho) = MkSChirho (TChirho b_chirho k4_chirho y_chirho)\n",
    );
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010, PolyKinds, NoCUSKs #-}\nmodule IncompleteRigidChirho where\ndata TChirho (a_chirho :: k_chirho) b_chirho = MkTChirho a_chirho\n",
    );
}

#[test]
fn kind_publication_honors_default_edition_and_legacy_completeness_chirho() {
    assert_compile_success_chirho(
        "DefaultGeneralizationChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule DefaultGeneralizationChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\nimport Data.Type.Equality ((:~:))\ndata AppChirho f_chirho a_chirho = MkAppChirho (f_chirho a_chirho)\ntype CatChirho k_chirho = k_chirho -> k_chirho -> Type\ndata FreeCatChirho :: CatChirho k_chirho -> CatChirho k_chirho\nappChirho :: AppChirho Proxy Maybe\nappChirho = undefined\nfreeChirho :: FreeCatChirho (:~:) Maybe Maybe\nfreeChirho = undefined\n",
    );
    assert_compile_success_chirho(
        "RecursiveNoPolyChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010, NoPolyKinds #-}\nmodule RecursiveNoPolyChirho where\ndata T1Chirho a_chirho = MkT1Chirho T2Chirho\ndata T2Chirho = MkT2Chirho (T1Chirho Maybe)\n",
    );
    assert_compile_success_chirho(
        "RecursivePolyChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010, PolyKinds #-}\nmodule RecursivePolyChirho where\ndata T1Chirho a_chirho = MkT1Chirho T2Chirho\ndata T2Chirho = MkT2Chirho (T1Chirho Maybe)\n",
    );
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE Haskell2010, NoPolyKinds, StandaloneKindSignatures #-}\nmodule RecursiveStandaloneChirho where\nimport Data.Kind (Type)\ndata T1Chirho a_chirho = MkT1Chirho T2Chirho\ntype T2Chirho :: Type\ndata T2Chirho = MkT2Chirho (T1Chirho Maybe)\n",
    );
}

#[test]
fn higher_kinded_forall_shadowing_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
retainChirho :: fChirho Int -> (forall fChirho. fChirho -> fChirho) -> fChirho Int
retainChirho valueChirho _ = valueChirho
retainLaterChirho :: (forall fChirho. fChirho -> fChirho) -> fChirho Int -> fChirho Int
retainLaterChirho _ valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho [42] id))
  print (sum (retainLaterChirho id [7]))
"#;
    assert_execution_chirho(source_chirho, "42\n7\n");
}

#[test]
fn required_higher_kinded_forall_shadowing_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, RequiredTypeArguments #-}
module Main where
retainChirho :: fChirho Int -> (forall fChirho -> fChirho -> fChirho) -> fChirho Int
retainChirho valueChirho _ = valueChirho
retainLaterChirho :: (forall fChirho -> fChirho -> fChirho) -> fChirho Int -> fChirho Int
retainLaterChirho _ valueChirho = valueChirho
requiredIdChirho :: forall aChirho -> aChirho -> aChirho
requiredIdChirho typeChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho [42] requiredIdChirho))
  print (sum (retainLaterChirho requiredIdChirho [7]))
"#;
    assert_execution_chirho(source_chirho, "42\n7\n");
}

#[test]
fn kind_annotated_forall_shadowing_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, ScopedTypeVariables, TypeApplications #-}
module Main where
import Data.Kind (Type)
data ProxyChirho (aChirho :: kChirho) = ProxyChirho
retainChirho :: forall kChirho (fChirho :: kChirho -> Type) (aChirho :: kChirho). fChirho aChirho -> (forall kChirho (aChirho :: kChirho). ProxyChirho aChirho -> ProxyChirho aChirho) -> fChirho aChirho
retainChirho valueChirho _ = valueChirho
proxyIdChirho :: forall kChirho (aChirho :: kChirho). ProxyChirho aChirho -> ProxyChirho aChirho
proxyIdChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho @Type @[] @Int [42] proxyIdChirho))
  print (maybe False id (retainChirho @Type @Maybe @Bool (Just True) proxyIdChirho))
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n");
}

#[test]
fn quantified_constraint_kind_scope_runs_on_every_engine_chirho() {
    // This identical source independently ran under GHC 9.14.1.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE QuantifiedConstraints, FlexibleInstances, UndecidableInstances, MonoLocalBinds #-}
module Main where
class WitnessChirho aChirho
instance WitnessChirho aChirho
retainChirho :: (forall fChirho. WitnessChirho (fChirho Int)) => fChirho -> fChirho
retainChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (retainChirho (42 :: Int))
  print (retainChirho True)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n");
}

#[test]
fn a_shadow_does_not_hide_a_genuine_kind_mismatch_chirho() {
    // GHC 9.14.1 reports GHC-83865: a Type variable cannot also take an argument.
    let source_chirho = r#"{-# LANGUAGE RankNTypes #-}
module BadKindScopeChirho where
badChirho :: (forall fChirho. fChirho -> fChirho Int) -> Bool
badChirho _ = True
"#;
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "BadKindScopeChirho.hs",
    )
    .map(|_| ())
    .expect_err("the same bound variable cannot have both Type and Type -> Type kinds");
    let text_chirho = errors_chirho.to_string();
    assert!(text_chirho.contains("kind mismatch"), "{text_chirho}");
}

#[test]
fn identity_function_infers_chirho() {
    assert_compile_success_chirho(
        "IdentityFunctionChirho.hs",
        "module IdentityFunctionChirho where\nfChirho xChirho = xChirho\n",
    );
}

#[test]
fn data_type_with_constructors_infers_chirho() {
    assert_compile_success_chirho(
        "DataTypeConstructorsChirho.hs",
        "module DataTypeConstructorsChirho where\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho\n",
    );
}

#[test]
fn if_expression_infers_chirho() {
    assert_compile_success_chirho(
        "IfExpressionChirho.hs",
        "module IfExpressionChirho where\ngChirho xChirho = if True then xChirho else xChirho\n",
    );
}

#[test]
fn list_literal_infers_chirho() {
    assert_compile_success_chirho(
        "ListLiteralChirho.hs",
        "module ListLiteralChirho where\nxsChirho = [1, 2, 3]\n",
    );
}

#[test]
fn tuple_literal_infers_chirho() {
    assert_compile_success_chirho(
        "TupleLiteralChirho.hs",
        "module TupleLiteralChirho where\npChirho = (1, True)\n",
    );
}

#[test]
fn let_expression_infers_chirho() {
    assert_compile_success_chirho(
        "LetExpressionChirho.hs",
        "module LetExpressionChirho where\nhChirho = let { yChirho = 42 } in 42\n",
    );
}

#[test]
fn lambda_expression_infers_chirho() {
    assert_compile_success_chirho(
        "LambdaExpressionChirho.hs",
        "module LambdaExpressionChirho where\nkChirho = \\xChirho -> xChirho\n",
    );
}

#[test]
fn negative_type_error_reports_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module NegativeTypeErrorChirho where\nbadChirho = if 42 then 1 else 2\n",
        &mut source_map_chirho,
        "NegativeTypeErrorChirho.hs",
    );
    assert!(
        result_chirho.is_err(),
        "expected compile_source_chirho to report a type error"
    );
}

#[test]
fn signature_type_arguments_follow_source_order_on_every_engine_chirho() {
    assert_execution_chirho(SOURCE_ORDER_CHIRHO, "42\n7\n11\nTrue\n");
}

#[test]
fn nested_forall_shadowing_preserves_outer_polymorphism_on_every_engine_chirho() {
    // Independently run, unchanged, under GHC 9.14.1. The outer variable is
    // encountered before a shadow, after one, first inside another binder,
    // and in the enclosing ScopedTypeVariables environment respectively.
    let source_chirho = r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables, TypeApplications #-}
module Main where
preserveChirho :: aChirho -> (forall aChirho. aChirho -> aChirho) -> aChirho
preserveChirho valueChirho _ = valueChirho
preserveLaterChirho :: (forall aChirho. aChirho -> aChirho) -> aChirho -> aChirho
preserveLaterChirho _ valueChirho = valueChirho
retainFreeChirho :: (forall boundChirho. freeChirho -> boundChirho -> freeChirho) -> freeChirho -> freeChirho
retainFreeChirho functionChirho valueChirho = functionChirho valueChirho ()
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho valueChirho = innerChirho id valueChirho
  where
    innerChirho :: (forall aChirho. aChirho -> aChirho) -> aChirho -> aChirho
    innerChirho _ resultChirho = resultChirho
main :: IO ()
main = do
  print (preserveChirho (42 :: Int) id)
  print (preserveChirho True id)
  print (preserveLaterChirho id (7 :: Int))
  print (preserveLaterChirho id False)
  print (retainFreeChirho @Int const 11)
  print (retainFreeChirho @Bool const True)
  print (outerChirho @Int 13)
  print (outerChirho @Bool False)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n7\nFalse\n11\nTrue\n13\nFalse\n");
}

#[test]
fn required_forall_shadowing_preserves_outer_polymorphism_on_every_engine_chirho() {
    // The same source independently produced this output under GHC 9.14.1.
    let source_chirho = r#"{-# LANGUAGE RankNTypes, RequiredTypeArguments #-}
module Main where
preserveChirho :: aChirho -> (forall aChirho -> aChirho -> aChirho) -> aChirho
preserveChirho valueChirho _ = valueChirho
preserveLaterChirho :: (forall aChirho -> aChirho -> aChirho) -> aChirho -> aChirho
preserveLaterChirho _ valueChirho = valueChirho
requiredIdChirho :: forall aChirho -> aChirho -> aChirho
requiredIdChirho typeChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (preserveChirho (42 :: Int) requiredIdChirho)
  print (preserveChirho True requiredIdChirho)
  print (preserveLaterChirho requiredIdChirho (7 :: Int))
  print (preserveLaterChirho requiredIdChirho False)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\n7\nFalse\n");
}

#[test]
fn signature_scope_boundaries_preserve_type_application_on_every_engine_chirho() {
    // GHC 9.14.1 oracle: parentheses quantify once, explicit local binders
    // shadow enclosing ones, and consecutive groups retain distinct identities.
    let source_chirho = r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables, TypeApplications, AllowAmbiguousTypes #-}
module Main where
parenthesizedChirho :: (forall aChirho. aChirho -> aChirho)
parenthesizedChirho valueChirho = valueChirho
outerShadowChirho :: forall aChirho. aChirho -> (Bool, aChirho)
outerShadowChirho valueChirho = (innerChirho True, valueChirho)
  where
    innerChirho :: forall aChirho. aChirho -> aChirho
    innerChirho resultChirho = resultChirho
chainChirho :: forall aChirho. forall aChirho. aChirho -> aChirho
chainChirho valueChirho = valueChirho
leadingChirho :: forall aChirho. Eq aChirho => forall aChirho. Eq aChirho => aChirho -> aChirho
leadingChirho valueChirho = valueChirho
main :: IO ()
main = do
  print (parenthesizedChirho @Int 42)
  print (parenthesizedChirho @Bool True)
  print (fst (outerShadowChirho @Int 7))
  print (snd (outerShadowChirho @Int 7))
  print (chainChirho @Int @Bool False)
  print (leadingChirho @Int @Bool True)
"#;
    assert_execution_chirho(source_chirho, "42\nTrue\nTrue\n7\nFalse\nTrue\n");
}

#[test]
fn result_forall_predicates_keep_their_shadowed_scope_chirho() {
    let source_chirho = r#"{-# LANGUAGE RankNTypes, TypeApplications #-}
module Main where
equalLaterChirho :: Int -> forall aChirho. Eq aChirho => aChirho -> aChirho -> Bool
equalLaterChirho _ leftChirho rightChirho = leftChirho == rightChirho
shadowLaterChirho :: forall aChirho. aChirho -> forall aChirho. Eq aChirho => aChirho -> aChirho -> Bool
shadowLaterChirho _ leftChirho rightChirho = leftChirho == rightChirho
main :: IO ()
main = do
  print (equalLaterChirho 0 @Int 3 3)
  print (equalLaterChirho 0 @Bool True False)
  print (shadowLaterChirho @Int 0 @Bool True True)
  print (shadowLaterChirho @Bool True @Int 3 7)
"#;
    assert_execution_chirho(source_chirho, "True\nFalse\nTrue\nFalse\n");
}

#[test]
fn only_the_syntactically_outermost_forall_scopes_over_the_definition_chirho() {
    // GHC 9.14.1 rejects each local signature below: parentheses and a second
    // forall group do not bring those signature variables into the definition.
    for source_chirho in [
        r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables #-}
module ParenScopeChirho where
parenthesizedChirho :: (forall aChirho. aChirho -> aChirho)
parenthesizedChirho valueChirho = localChirho
  where
    localChirho :: aChirho
    localChirho = valueChirho
"#,
        r#"{-# LANGUAGE RankNTypes, ScopedTypeVariables #-}
module LaterScopeChirho where
laterChirho :: forall aChirho. forall bChirho. aChirho -> bChirho -> bChirho
laterChirho _ valueChirho = localChirho
  where
    localChirho :: bChirho
    localChirho = valueChirho
"#,
    ] {
        let errors_chirho = haskelujah_driver::typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "SignatureScopeChirho.hs",
        )
        .map(|_| ())
        .expect_err("a non-scoping signature binder cannot capture the local type variable");
        let text_chirho = errors_chirho.to_string();
        assert!(
            text_chirho.contains("type mismatch") && text_chirho.contains("rigid"),
            "{text_chirho}"
        );
    }
}

#[test]
fn nested_forall_result_cannot_escape_as_an_arbitrary_outer_type_chirho() {
    // GHC 9.14.1 rejects this with GHC-25897: outer aChirho is rigid, not Bool.
    let source_chirho = r#"{-# LANGUAGE RankNTypes #-}
module ForallLeakChirho where
leakChirho :: aChirho -> (forall aChirho. aChirho -> aChirho) -> aChirho
leakChirho _ functionChirho = functionChirho True
"#;
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ForallLeakChirho.hs",
    )
    .map(|_| ())
    .expect_err("an arbitrary outer a cannot be implemented with Bool");
    let text_chirho = errors_chirho.to_string();
    assert!(
        text_chirho.contains("type mismatch")
            && text_chirho.contains("Bool")
            && text_chirho.contains("rigid"),
        "{text_chirho}"
    );
}

#[test]
fn prefix_list_constructor_in_a_record_field_executes_chirho() {
    // GHC 9.14.1 independently prints kept; [] must not acquire a fake element.
    let source_chirho = r#"module Main where
data HolderChirho = HolderChirho { fieldChirho :: [] Char }
main :: IO ()
main = putStrLn (fieldChirho (HolderChirho "kept"))
"#;
    assert_execution_chirho(source_chirho, "kept\n");
}

#[test]
fn nested_list_function_in_a_record_field_executes_chirho() {
    // GHC 9.14.1 independently prints 42; the inner ] cannot close the outer list.
    let source_chirho = r#"module Main where
data HolderChirho = HolderChirho { fieldChirho :: [[Int] -> Int] }
main :: IO ()
main = case fieldChirho (HolderChirho [\_ -> 42]) of
  firstChirho : _ -> print (firstChirho [1, 2])
  [] -> print (0 :: Int)
"#;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn gadt_record_list_types_and_following_signature_execute_chirho() {
    let source_chirho = r#"{-# LANGUAGE GADTs #-}
module Main where
data HolderChirho where
  HolderChirho :: { textChirho :: [] Char, functionsChirho :: [[Int] -> Int] } -> HolderChirho
main :: IO ()
main = do
  let holderChirho = HolderChirho "kept" [\_ -> 42]
  putStrLn (textChirho holderChirho)
  case functionsChirho holderChirho of
    firstChirho : _ -> print (firstChirho [1, 2])
    [] -> print (0 :: Int)
"#;
    assert_execution_chirho(source_chirho, "kept\n42\n");
}

#[test]
fn bare_list_constructor_is_not_a_saturated_record_field_type_chirho() {
    for declaration_chirho in [
        "data HolderChirho = HolderChirho { fieldChirho :: [] }",
        "data HolderChirho where\n  HolderChirho :: { fieldChirho :: [] } -> HolderChirho",
        "data HolderChirho = HolderChirho { fieldChirho :: [] Int Bool }",
        "data HolderChirho where\n  HolderChirho :: { fieldChirho :: [] Int Bool } -> HolderChirho",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE GADTs #-}}\nmodule UnsaturatedListChirho where\n{declaration_chirho}\n"
        );
        let error_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "UnsaturatedListChirho.hs",
        )
        .map(|_| ())
        .expect_err("GHC-83865: [] needs exactly one argument before it is a field type");
        let message_chirho = error_chirho.to_string();
        assert!(message_chirho.contains("kind mismatch"), "{message_chirho}");
    }
}

#[test]
fn declaration_kind_contracts_execute_on_every_engine_chirho() {
    // Identical source independently executed under GHC 9.14.1: 42 / 7 / 11 / 13.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, KindSignatures, PolyKinds, StandaloneKindSignatures #-}
module Main where
import Data.Kind (Type)
data InlineChirho (aChirho :: kChirho) (bChirho :: kChirho) :: kChirho -> Type where
  MkInlineChirho :: InlineChirho aChirho bChirho aChirho
type CompleteChirho :: Type -> Type
data CompleteChirho aChirho = MkCompleteChirho aChirho
type CombinedChirho :: (Type -> Type) -> Type -> Type
data CombinedChirho (fChirho :: Type -> Type) aChirho :: Type where
  MkCombinedChirho :: fChirho aChirho -> CombinedChirho fChirho aChirho
type WrapperChirho :: Type -> Type
newtype WrapperChirho (aChirho :: Type) :: Type where
  MkWrapperChirho :: aChirho -> WrapperChirho aChirho
main :: IO ()
main = do
  case (MkInlineChirho :: InlineChirho Int Bool Int) of
    MkInlineChirho -> print (42 :: Int)
  case MkCompleteChirho (7 :: Int) of
    MkCompleteChirho valueChirho -> print valueChirho
  case MkCombinedChirho [11 :: Int] of
    MkCombinedChirho valuesChirho -> print (sum valuesChirho)
  case MkWrapperChirho (13 :: Int) of
    MkWrapperChirho valueChirho -> print valueChirho
"#;
    assert_execution_chirho(source_chirho, "42\n7\n11\n13\n");
}

#[test]
fn standalone_kind_contracts_reject_mismatched_heads_and_tails_chirho() {
    // Each case is GHC-83865. The combined case was accepted after its
    // standalone signature was silently replaced by the inline Type tail.
    for declarations_chirho in [
        "type BoxChirho :: (Type -> Type) -> Type\ndata BoxChirho (aChirho :: Type) = MkBoxChirho aChirho",
        "type BoxChirho :: Type -> Type\ndata BoxChirho :: Type where\n  MkBoxChirho :: BoxChirho",
        "type BoxChirho :: Type\ndata BoxChirho :: Type -> Type where\n  MkBoxChirho :: BoxChirho Int",
        "type BoxChirho :: Type -> Type\ndata BoxChirho where\n  MkBoxChirho :: BoxChirho Int",
        "type BoxChirho :: Type -> Type -> Type\ndata BoxChirho aChirho where\n  MkBoxChirho :: BoxChirho aChirho bChirho",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE GADTs, KindSignatures, StandaloneKindSignatures #-}}\nmodule BadDeclarationKindChirho where\nimport Data.Kind (Type)\n{declarations_chirho}\n"
        );
        let error_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "BadDeclarationKindChirho.hs",
        )
        .map(|_| ())
        .expect_err("a complete signature must constrain the declaration head and result");
        let message_chirho = error_chirho.to_string();
        assert!(
            message_chirho.contains("kind mismatch in data declaration signature"),
            "{message_chirho}"
        );
    }
}

const INVISIBLE_DATA_BINDERS_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeAbstractions, StandaloneKindSignatures, PolyKinds #-}
module Main where
import Data.Kind (Type)
type BoxChirho :: forall kChirho. Type -> Type
data BoxChirho @jChirho aChirho = MkBoxChirho { boxValueChirho :: aChirho } deriving (Eq, Show)
type WrapperChirho :: forall kChirho. Type -> Type
newtype WrapperChirho @jChirho aChirho = MkWrapperChirho { wrapperValueChirho :: aChirho } deriving (Eq, Show)
boxChirho :: BoxChirho Int
boxChirho = MkBoxChirho 42
wrapperChirho :: WrapperChirho Int
wrapperChirho = MkWrapperChirho 7
main :: IO ()
main = do
  print (boxValueChirho boxChirho)
  print (wrapperValueChirho wrapperChirho)
  print (boxChirho == MkBoxChirho 42)
  print (wrapperChirho == MkWrapperChirho 7)
  print boxChirho
  print wrapperChirho
"#;

#[test]
fn invisible_data_heads_and_record_fields_execute_on_every_engine_chirho() {
    // The field-read source independently runs 42/7 under GHC 9.14.1.
    // Ordinary polymorphic record Eq/Show execution has a separate existing
    // failure; this test must not assert our wrong output as its oracle.
    let source_chirho = INVISIBLE_DATA_BINDERS_CHIRHO
        .lines()
        .filter(|line_chirho| {
            !line_chirho.contains(" == ")
                && !matches!(*line_chirho, "  print boxChirho" | "  print wrapperChirho")
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert_execution_chirho(&source_chirho, "42\n7\n");
}

#[test]
fn invisible_data_heads_derived_instances_typecheck_chirho() {
    // GHC accepts the identical source. This asserts instance-head typing,
    // not runtime dispatch or formatting (separately recorded limitations).
    haskelujah_driver::typecheck_source_chirho(
        INVISIBLE_DATA_BINDERS_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "InvisibleDerivedInstancesChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn invisible_data_head_fields_reject_the_wrong_value_type_chirho() {
    for (old_chirho, new_chirho) in [
        ("boxChirho = MkBoxChirho 42", "boxChirho = MkBoxChirho True"),
        (
            "wrapperChirho = MkWrapperChirho 7",
            "wrapperChirho = MkWrapperChirho True",
        ),
    ] {
        // Each exact source mutation is rejected independently by GHC 9.14.1.
        let source_chirho = INVISIBLE_DATA_BINDERS_CHIRHO.replace(old_chirho, new_chirho);
        let error_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "WrongHeadFieldChirho.hs",
        )
        .map(|_| ())
        .expect_err("the visible Int argument must constrain the constructor field");
        let message_chirho = error_chirho.to_string();
        assert!(
            message_chirho.contains("type mismatch")
                && message_chirho.contains("Int")
                && message_chirho.contains("Bool"),
            "{message_chirho}"
        );
    }
}

#[test]
fn required_kind_binders_execute_on_every_engine_chirho() {
    // Identical source independently executed with GHC 9.14.1: 19.
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeAbstractions, StandaloneKindSignatures, PolyKinds, GADTs, DataKinds #-}
module Main where
import Data.Kind (Type)
type VisibleChirho :: forall (flagChirho :: Bool) -> Type
data VisibleChirho flagChirho = MkVisibleChirho Int
valueChirho :: VisibleChirho 'True
valueChirho = MkVisibleChirho 19
main :: IO ()
main = case valueChirho of
  MkVisibleChirho numberChirho -> print numberChirho
"#;
    assert_execution_chirho(source_chirho, "19\n");
}

fn assert_execution_chirho(source_chirho: &str, expected_chirho: &str) {
    use haskelujah_test_harness_chirho::native_chirho::{
        NativeBackendChirho, native_round_trip_chirho,
    };

    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "TypeOrderChirho.hs",
        None,
    )
    .expect("the GHC-accepted signature applications must compile and run");
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let (status_chirho, output_chirho) =
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .expect("bounded native compile, link and execution must succeed");
        assert_eq!(status_chirho, 0, "{backend_chirho:?}");
        assert_eq!(output_chirho, expected_chirho, "{backend_chirho:?}");
    }
}

#[test]
fn type_operator_fixity_agrees_with_constructor_values_on_every_engine_chirho() {
    // The unchanged source was run under GHC 9.14.1: 3 / True / 'c'.
    let source_chirho = r#"{-# LANGUAGE TypeOperators #-}
module Main where
infixl 4 :*:
infixl 3 :+:
data aChirho :*: bChirho = ProductChirho aChirho bChirho
data aChirho :+: bChirho = SumChirho aChirho bChirho
valueChirho :: Int :*: Bool :+: Char
valueChirho = SumChirho (ProductChirho 3 True) 'c'
main :: IO ()
main = case valueChirho of
  SumChirho (ProductChirho numberChirho flagChirho) letterChirho -> do
    print numberChirho
    print flagChirho
    print letterChirho
"#;
    assert_execution_chirho(source_chirho, "3\nTrue\n'c'\n");
}

#[test]
fn class_parameters_and_scoped_and_rank_n_binders_keep_their_type_arguments_chirho() {
    // This is a typechecking contract. The analogous custom class-method call
    // has a separate pre-existing STG dispatch limitation, so it is not being
    // presented as an execution test. GHC 9.14.1 accepts and runs this source.
    let source_chirho = r#"{-# LANGUAGE TypeApplications, ExplicitForAll, ScopedTypeVariables, RankNTypes #-}
module Main where
class KeepChirho aChirho where
  keepChirho :: bChirho -> aChirho -> bChirho
instance KeepChirho Int where
  keepChirho valueChirho _ = valueChirho
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho valueChirho = innerChirho @Bool True
  where
    innerChirho :: bChirho -> aChirho
    innerChirho _ = valueChirho
polyChirho :: (forall qChirho. qChirho -> qChirho) -> aChirho -> aChirho
polyChirho functionChirho valueChirho = functionChirho valueChirho
main :: IO ()
main = do
  print (keepChirho @Int @Bool True 3)
  print (outerChirho @Int 7)
  print (polyChirho @Char id 'x')
"#;
    haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ScopeOrderChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn explicit_type_arguments_do_not_accept_reversed_operand_types_chirho() {
    let source_chirho = SOURCE_ORDER_CHIRHO.replace(
        "implicitChirho @Int @(,) @Bool (42, True)",
        "implicitChirho @Int @(,) @Bool (True, 42)",
    );
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "TypeOrderMismatchChirho.hs",
    )
    .map(|_| ())
    .expect_err("the explicitly Int first component cannot be Bool");
    let text_chirho = format!("{errors_chirho}");
    assert!(
        text_chirho.contains("type mismatch")
            && text_chirho.contains("Int")
            && text_chirho.contains("Bool"),
        "{text_chirho}"
    );
}
