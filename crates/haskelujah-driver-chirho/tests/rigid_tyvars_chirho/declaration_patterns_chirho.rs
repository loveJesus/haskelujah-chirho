// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration producers must supply faithful known types. Impossible GADT
//! index equations retain their consistent components; fields/bodies still
//! meet their ordinary types. Sources have fresh GHC 9.14.1 receipts.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::{assert_accepted_chirho, assert_output_chirho, assert_rejected_with_chirho};

#[test]
fn declaration_heads_reach_known_type_patterns_chirho() {
    for (file_chirho, source_chirho) in [
        (
            "promoted_nil_instance_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data HChirho (xs_chirho :: [Type]) where
  HNilChirho :: HChirho '[]
  HOneChirho :: HChirho '[Int]
class CChirho (xs_chirho :: [Type]) where
  countChirho :: HChirho xs_chirho -> Int
instance CChirho '[] where
  countChirho HNilChirho = 0
main = print (countChirho HNilChirho)
"#,
        ),
        (
            "promoted_nil_mismatch_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data HChirho (xs_chirho :: [Type]) where
  HNilChirho :: HChirho '[]
  HOneChirho :: HChirho '[Int]
class CChirho (xs_chirho :: [Type]) where
  countChirho :: HChirho xs_chirho -> Int
instance CChirho '[] where
  countChirho HOneChirho = 0
main = pure ()
"#,
        ),
        (
            "promoted_nonempty_instance_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data HChirho (xs_chirho :: [Type]) where
  HNilChirho :: HChirho '[]
  HOneChirho :: HChirho '[Int]
class CChirho (xs_chirho :: [Type]) where
  countChirho :: HChirho xs_chirho -> Int
instance CChirho '[Int] where
  countChirho HOneChirho = 1
main = print (countChirho HOneChirho)
"#,
        ),
        (
            "parenthesized_promoted_nil_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data HChirho (xs_chirho :: [Type]) where
  HNilChirho :: HChirho '[]
  HOneChirho :: HChirho '[Int]
class CChirho (xs_chirho :: [Type]) where
  countChirho :: HChirho xs_chirho -> Int
instance CChirho ('[]) where
  countChirho HNilChirho = 0
main = print (countChirho HNilChirho)
"#,
        ),
        (
            "nested_promoted_instance_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data HChirho (xs_chirho :: [[Type]]) where
  HNestedChirho :: HChirho '[ '[Int]]
class CChirho (xs_chirho :: [[Type]]) where
  countChirho :: HChirho xs_chirho -> Int
instance CChirho '[ '[Int]] where
  countChirho HNestedChirho = 1
main = print (countChirho HNestedChirho)
"#,
        ),
        (
            "ordinary_list_constructor_head_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
class FChirho (f_chirho :: Type -> Type) where
  sizeChirho :: f_chirho Int -> Int
instance FChirho [] where
  sizeChirho _ = 0
main = print (sizeChirho ([1] :: [Int]))
"#,
        ),
        (
            "standalone_kind_head_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data SBoolChirho :: Bool -> Type where
  STrueChirho :: SBoolChirho 'True
  SFalseChirho :: SBoolChirho 'False
deriving instance Show (SBoolChirho b_chirho)
main = print (STrueChirho, SFalseChirho)
"#,
        ),
        (
            "standalone_specialized_head_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data BoxChirho a_chirho = BoxChirho a_chirho
deriving instance Show (BoxChirho Int)
main = print (BoxChirho (7 :: Int))
"#,
        ),
        (
            "standalone_written_context_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data BoxChirho a_chirho = BoxChirho a_chirho
deriving instance Show a_chirho => Show (BoxChirho a_chirho)
main = print (BoxChirho True)
"#,
        ),
        (
            "inline_kind_arity_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, FlexibleInstances, FlexibleContexts, StandaloneDeriving, UndecidableInstances #-}
module Main where
import Data.Kind (Type)
data BoxChirho :: Type -> Type where
  BoxChirho :: a_chirho -> BoxChirho a_chirho
  deriving (Eq, Show)
main = print (BoxChirho 'x', BoxChirho 'x' == BoxChirho 'y')
"#,
        ),
    ] {
        assert_accepted_chirho(file_chirho, source_chirho);
    }
}

#[test]
fn inaccessible_indices_keep_consistent_fields_chirho() {
    for (file_chirho, source_chirho) in [
        (
            "IA01_simple_constant_body_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: TChirho Int
  TBoolChirho :: TChirho Bool
fChirho :: TChirho Int -> Int
fChirho TIntChirho = 1
fChirho TBoolChirho = 2
main :: IO ()
main = print (fChirho TIntChirho)
"#,
        ),
        (
            "IA02_simple_field_body_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: Int -> TChirho Int
  TBoolChirho :: Bool -> TChirho Bool
fChirho :: TChirho Int -> Int
fChirho (TIntChirho nChirho) = nChirho
fChirho (TBoolChirho bChirho) = fromEnum bChirho
main :: IO ()
main = print (fChirho (TIntChirho 5))
"#,
        ),
        (
            "IA04_partial_result_constant_body_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data PChirho aChirho where
  PPairChirho :: bChirho -> PChirho (bChirho, Int)
  PSwapChirho :: cChirho -> PChirho (Int, cChirho)
hChirho :: PChirho (Bool, Int) -> Int
hChirho (PPairChirho bChirho) = fromEnum bChirho
hChirho (PSwapChirho _) = 0
main :: IO ()
main = print (hChirho (PPairChirho True))
"#,
        ),
        (
            "IA05_partial_result_field_body_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data PChirho aChirho where
  PPairChirho :: bChirho -> PChirho (bChirho, Int)
  PSwapChirho :: cChirho -> PChirho (Int, cChirho)
hChirho :: PChirho (Bool, Int) -> Int
hChirho (PPairChirho bChirho) = fromEnum bChirho
hChirho (PSwapChirho cChirho) = cChirho + 1
main :: IO ()
main = print (hChirho (PPairChirho True))
"#,
        ),
        (
            "IA05b_partial_result_field_body_flipped_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data PChirho aChirho where
  PPairChirho :: bChirho -> PChirho (bChirho, Int)
  PSwapChirho :: cChirho -> PChirho (Int, cChirho)
  PFlipChirho :: cChirho -> PChirho (cChirho, Bool)
kChirho :: PChirho (Int, Int) -> Int
kChirho (PPairChirho bChirho) = bChirho
kChirho (PFlipChirho cChirho) = cChirho + 1
main :: IO ()
main = print (kChirho (PPairChirho 4))
"#,
        ),
        (
            "IA06_nested_inaccessible_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: TChirho Int
  TBoolChirho :: TChirho Bool
fChirho :: Maybe (TChirho Int) -> Int
fChirho (Just TBoolChirho) = 0
fChirho _ = 1
main :: IO ()
main = print (fChirho (Just TIntChirho))
"#,
        ),
        (
            "IA08_promoted_index_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, DataKinds, KindSignatures #-}
module Main where
import Data.Kind (Type)
data SChirho :: Bool -> Type where
  STrueChirho :: SChirho 'True
  SFalseChirho :: SChirho 'False
fChirho :: SChirho 'True -> Int
fChirho STrueChirho = 1
fChirho SFalseChirho = 0
main :: IO ()
main = print (fChirho STrueChirho)
"#,
        ),
        (
            "IA09_instance_method_arm_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, DataKinds, KindSignatures #-}
module Main where
import Data.Kind (Type)
data SChirho :: Bool -> Type where
  STrueChirho :: SChirho 'True
  SFalseChirho :: SChirho 'False
class CChirho (bChirho :: Bool) where
  cChirho :: SChirho bChirho -> Int
instance CChirho 'True where
  cChirho STrueChirho = 1
  cChirho SFalseChirho = 0
main :: IO ()
main = print (cChirho STrueChirho)
"#,
        ),
        (
            "IA10_partial_consistent_child_literal_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data PChirho aChirho where
  PPairChirho :: bChirho -> PChirho (bChirho, Int)
  PSwapChirho :: cChirho -> PChirho (Int, cChirho)
  PFlipChirho :: cChirho -> PChirho (cChirho, Bool)
hChirho :: PChirho (Bool, Int) -> Int
hChirho (PPairChirho bChirho) = fromEnum bChirho
hChirho (PSwapChirho 5) = 0
hChirho _ = 9
main :: IO ()
main = print (hChirho (PPairChirho True))
"#,
        ),
    ] {
        assert_accepted_chirho(file_chirho, source_chirho);
    }
}

#[test]
fn inaccessible_indices_do_not_hide_mismatches_chirho() {
    for (file_chirho, source_chirho) in [
        (
            "MR01_outer_head_mismatch_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: TChirho Int
  TBoolChirho :: TChirho Bool
fChirho :: TChirho Int -> Int
fChirho TIntChirho = 1
fChirho Nothing = 0
main :: IO ()
main = print (fChirho TIntChirho)
"#,
        ),
        (
            "MR02_child_mismatch_accessible_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: Int -> TChirho Int
  TBoolChirho :: Bool -> TChirho Bool
fChirho :: TChirho Int -> Int
fChirho (TIntChirho True) = 0
fChirho _ = 1
main :: IO ()
main = print (fChirho (TIntChirho 5))
"#,
        ),
        (
            "MR03_child_mismatch_inaccessible_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: Int -> TChirho Int
  TBoolChirho :: Bool -> TChirho Bool
fChirho :: TChirho Int -> Int
fChirho (TIntChirho nChirho) = nChirho
fChirho (TBoolChirho 'c') = 0
main :: IO ()
main = print (fChirho (TIntChirho 5))
"#,
        ),
        (
            "MR04_wrong_body_inaccessible_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data TChirho aChirho where
  TIntChirho :: TChirho Int
  TBoolChirho :: TChirho Bool
fChirho :: TChirho Int -> Int
fChirho TIntChirho = 1
fChirho TBoolChirho = 'c'
main :: IO ()
main = print (fChirho TIntChirho)
"#,
        ),
        (
            "MR05_partial_consistent_child_mismatch_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where
data PChirho aChirho where
  PPairChirho :: bChirho -> PChirho (bChirho, Int)
  PSwapChirho :: cChirho -> PChirho (Int, cChirho)
  PFlipChirho :: cChirho -> PChirho (cChirho, Bool)
hChirho :: PChirho (Bool, Int) -> Int
hChirho (PPairChirho bChirho) = fromEnum bChirho
hChirho (PSwapChirho 'x') = 0
hChirho _ = 9
main :: IO ()
main = print (hChirho (PPairChirho True))
"#,
        ),
        (
            "C2_promoted_cons_refinement_negative_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, TypeOperators #-}
module Main where
import Data.Kind (Type)
data HListChirho :: [Type] -> Type where
  HNilChirho :: HListChirho '[]
  (:::) :: tChirho -> HListChirho tsChirho -> HListChirho (tChirho ': tsChirho)
infixr 5 :::
headChirho :: HListChirho '[Int] -> Bool
headChirho (xChirho ::: _) = xChirho
main :: IO ()
main = print (headChirho (5 ::: HNilChirho))
"#,
        ),
        (
            "C3_instance_head_nil_negative_chirho.hs",
            r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, TypeOperators #-}
module Main where
import Data.Kind (Type)
data HListChirho :: [Type] -> Type where
  HNilChirho :: HListChirho '[]
  (:::) :: tChirho -> HListChirho tsChirho -> HListChirho (tChirho ': tsChirho)
infixr 5 :::
class CChirho (tsChirho :: [Type]) where
  cChirho :: HListChirho tsChirho -> Int
instance CChirho '[] where
  cChirho HNilChirho = 'x'
main :: IO ()
main = print (cChirho HNilChirho)
"#,
        ),
    ] {
        assert_rejected_with_chirho(file_chirho, source_chirho, "type mismatch");
    }
}

#[test]
fn promoted_cons_refines_fields_without_hiding_body_mismatches_chirho() {
    assert_output_chirho(
        "C1_promoted_cons_refinement_positive_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, TypeOperators #-}
module Main where
import Data.Kind (Type)
data HListChirho :: [Type] -> Type where
  HNilChirho :: HListChirho '[]
  (:::) :: tChirho -> HListChirho tsChirho -> HListChirho (tChirho ': tsChirho)
infixr 5 :::
headChirho :: HListChirho '[Int] -> Int
headChirho (xChirho ::: _) = xChirho
main :: IO ()
main = print (headChirho (5 ::: HNilChirho))
"#,
        "5\n",
    );
}

#[test]
fn deriving_keeps_the_written_specialization_and_context_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE StandaloneDeriving, FlexibleInstances #-}
module DerivingHeadsChirho where
data BoxChirho aChirho = BoxChirho aChirho
deriving instance Eq aChirho => Eq (BoxChirho aChirho)
deriving instance Show (BoxChirho Int)
"#;
    let file_chirho = haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = haskelujah_parser_chirho::cst_parser_chirho::parse_to_cst_chirho(
        source_chirho,
        file_chirho,
    );
    let module_chirho =
        haskelujah_parser_chirho::lower_chirho::lower_module_chirho(&cst_chirho, file_chirho);
    let written_chirho: Vec<_> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| {
            if let DeclChirho::StandaloneDerivingDeclChirho {
                class_chirho,
                context_chirho,
                types_chirho,
                ..
            } = decl_chirho
            {
                Some((
                    class_chirho.full_name_chirho(),
                    context_chirho.clone(),
                    types_chirho.clone(),
                ))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(written_chirho.len(), 2);
    let result_chirho = haskelujah_typing_chirho::derive_instances_chirho(&module_chirho);
    assert!(
        result_chirho.warnings_chirho.is_empty(),
        "{:?}",
        result_chirho.warnings_chirho
    );
    let generated_chirho: Vec<_> = result_chirho
        .instances_chirho
        .into_iter()
        .filter_map(|decl_chirho| {
            if let DeclChirho::InstanceDeclChirho {
                class_chirho,
                context_chirho,
                types_chirho,
                ..
            } = decl_chirho
            {
                Some((
                    class_chirho.full_name_chirho(),
                    context_chirho,
                    types_chirho,
                ))
            } else {
                None
            }
        })
        .collect();
    // Instance identity/context are semantic output, independent of generated
    // method names, spans, occurrence ids, and later dictionary selection.
    assert_eq!(generated_chirho, written_chirho);
}

#[test]
fn infix_constructor_values_keep_their_fixity_chirho() {
    assert_output_chirho(
        "infix_constructor_values_chirho.hs",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data TreeChirho = LeafChirho Int | TreeChirho :*: TreeChirho
infixl 6 :*:
scoreChirho :: TreeChirho -> Int
scoreChirho (LeafChirho nChirho) = nChirho
scoreChirho (leftChirho :*: rightChirho) = 10 * scoreChirho leftChirho + scoreChirho rightChirho
main :: IO ()
main = print (scoreChirho (LeafChirho 1 :*: LeafChirho 2 :*: LeafChirho 3))
"#,
        "123\n",
    );
}
