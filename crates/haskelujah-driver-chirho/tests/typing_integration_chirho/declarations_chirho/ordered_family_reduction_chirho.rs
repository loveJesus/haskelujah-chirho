// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordered closed-family controls, copied byte for byte from fresh GHC 9.14.1 oracle files.
//! Paired abstract negatives prevent a fallback from reducing before its blocker is apart.
//! workflow: language-features-chirho/stuck-family-equalities-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

#[test]
fn a03_wildcards_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle A03 (686a): every wildcard is its own variable; two wildcards never force
-- equality, a repeated named variable does. Predicted GHC 9.14.1 output: True, True, False, True.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures, ScopedTypeVariables #-}
module Main where

import Data.Proxy (Proxy (..))

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

type family BothChirho aChirho bChirho :: Bool where
  BothChirho _ _ = True

type family PickChirho aChirho bChirho cChirho :: Bool where
  PickChirho aChirho _ aChirho = True
  PickChirho _ _ _ = False

main :: IO ()
main = do
  print (boolValChirho (Proxy :: Proxy (BothChirho Char Bool)))
  print (boolValChirho (Proxy :: Proxy (PickChirho Int Char Int)))
  print (boolValChirho (Proxy :: Proxy (PickChirho Int Char Bool)))
  print (boolValChirho (Proxy :: Proxy (PickChirho Int Int Int)))
"###;
    assert_prints_everywhere_chirho(
        "A03WildcardsChirho.hs",
        source_chirho,
        "True\nTrue\nFalse\nTrue\n",
    );
}

#[test]
fn b01_local_names_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B01 (686b): equation variables that are not declaration binders, and an
-- explicit equation forall. Registration today turns such names into rigid constructors.
-- Predicted GHC 9.14.1 output: 3, 3, 3.
{-# LANGUAGE DataKinds, TypeFamilies, TypeOperators, PolyKinds, KindSignatures, ScopedTypeVariables, ExplicitForAll #-}
module Main where

import Data.Kind (Type)
import Data.Proxy (Proxy (..))

data NatChirho = ZeroChirho | SuccChirho NatChirho

class KnownNatChirho (nChirho :: NatChirho) where
  natValChirho :: Proxy nChirho -> Int

instance KnownNatChirho ZeroChirho where
  natValChirho _ = 0

instance KnownNatChirho nChirho => KnownNatChirho (SuccChirho nChirho) where
  natValChirho _ = 1 + natValChirho (Proxy :: Proxy nChirho)

type family LenChirho (xsChirho :: [Type]) :: NatChirho where
  LenChirho '[] = ZeroChirho
  LenChirho (yChirho ': ysChirho) = SuccChirho (LenChirho ysChirho)

type family AppendChirho (xsChirho :: [Type]) (ysChirho :: [Type]) :: [Type] where
  AppendChirho '[] zsChirho = zsChirho
  AppendChirho (wChirho ': wsChirho) zsChirho = wChirho ': AppendChirho wsChirho zsChirho

type family ElemChirho aChirho where
  forall xChirho. ElemChirho [xChirho] = xChirho

main :: IO ()
main = do
  print (natValChirho (Proxy :: Proxy (LenChirho '[Int, Bool, Char])))
  print (natValChirho (Proxy :: Proxy (LenChirho (AppendChirho '[Int] '[Bool, Char]))))
  print (3 :: ElemChirho [Int])
"###;
    assert_prints_everywhere_chirho("B01LocalNamesChirho.hs", source_chirho, "3\n3\n3\n");
}

#[test]
fn b03_deferred_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B03 (686b): an application stuck on an unknown must wait, then reduce.
-- First-match today reduces `FChirho a` to Char while converting the signature, so the call
-- with a := Int is predicted to fail today. Predicted GHC 9.14.1 output: True.
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))

type family FChirho aChirho where
  FChirho Int = Bool
  FChirho aChirho = Char

hChirho :: Proxy aChirho -> FChirho aChirho -> FChirho aChirho
hChirho _ valueChirho = valueChirho

main :: IO ()
main = print (hChirho (Proxy :: Proxy Int) True)
"###;
    assert_prints_everywhere_chirho("B03DeferredChirho.hs", source_chirho, "True\n");
}

#[test]
fn b07_frob_twin_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B07 (686b): the GHC-valid twin of T8044. Each clause's GADT refinement makes
-- `FrobChirho a` reducible: Int for XIntChirho, Char for the others, since Int is apart from
-- Bool and Char. Predicted today: rejected (the fallback equation's `xChirho` is a rigid
-- constructor). Predicted with the binder fix alone: rejected (the signature reduces eagerly
-- to Char). Predicted GHC 9.14.1 output: int, char, char.
{-# LANGUAGE GADTs, TypeFamilies #-}
module Main where

data XChirho aChirho where
  XIntChirho :: XChirho Int
  XBoolChirho :: XChirho Bool
  XCharChirho :: XChirho Char

type family FrobChirho aChirho where
  FrobChirho Int = Int
  FrobChirho xChirho = Char

frobChirho :: XChirho aChirho -> XChirho (FrobChirho aChirho)
frobChirho XIntChirho = XIntChirho
frobChirho XBoolChirho = XCharChirho
frobChirho XCharChirho = XCharChirho

showXChirho :: XChirho bChirho -> String
showXChirho XIntChirho = "int"
showXChirho XBoolChirho = "bool"
showXChirho XCharChirho = "char"

main :: IO ()
main = do
  putStrLn (showXChirho (frobChirho XIntChirho))
  putStrLn (showXChirho (frobChirho XBoolChirho))
  putStrLn (showXChirho (frobChirho XCharChirho))
"###;
    assert_prints_everywhere_chirho("B07FrobTwinChirho.hs", source_chirho, "int\nchar\nchar\n");
}

#[test]
fn c02_same_wild_refl_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C02 (tasklist, GADT twin of C01): under the given a ~ b the first equation matches. Predicted GHC 9.14.1 output: True, False.
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures, GADTs #-}
module Main where

import Data.Proxy (Proxy (..))

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

type family SameChirho aChirho bChirho :: Bool where
  SameChirho aChirho aChirho = True
  SameChirho _ _ = False

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho

sameReflChirho :: EqualChirho aChirho bChirho -> Proxy (SameChirho aChirho bChirho)
sameReflChirho ReflChirho = (Proxy :: Proxy True)

main :: IO ()
main = do
  print (boolValChirho (sameReflChirho (ReflChirho :: EqualChirho Int Int)))
  print (boolValChirho (Proxy :: Proxy (SameChirho Int Bool)))
"###;
    assert_prints_everywhere_chirho("C02SameWildReflChirho.hs", source_chirho, "True\nFalse\n");
}

#[test]
fn c07_shared_stuck_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C07 (tasklist): the same stuck application twice. With one shared flattening variable the target is apart from `Two Int Bool`, so the fallback fires. Predicted GHC 9.14.1 output: 3 (UNKNOWN if GHC does not share).
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

twoChirho :: Proxy aChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho aChirho)
twoChirho _ = 3

main :: IO ()
main = print (twoChirho (Proxy :: Proxy Bool))
"###;
    assert_prints_everywhere_chirho("C07SharedStuckChirho.hs", source_chirho, "3\n");
}

#[test]
fn b04_stuck_skolem_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle B04 (686b negative): `SameChirho a b` on two rigid variables is stuck, because
-- the first equation is not apart from it. Predicted GHC 9.14.1: rejected, couldn't match
-- `SameChirho a b` with False. Predicted today: accepted (first-match picks False).
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures #-}
module B04StuckSkolemChirho where

import Data.Proxy (Proxy (..))

type family SameChirho aChirho bChirho :: Bool where
  SameChirho aChirho aChirho = True
  SameChirho aChirho bChirho = False

gChirho :: Proxy aChirho -> Proxy bChirho -> Proxy (SameChirho aChirho bChirho)
gChirho _ _ = (Proxy :: Proxy False)
"###;
    rejected_like_ghc_chirho(source_chirho, "B04StuckSkolemChirho.hs", &["SameChirho"]);
}

#[test]
fn c03_cross_abstract_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C03 (tasklist): `Cross a Int = a; Cross Bool a = a` are incompatible once BOTH sides, right sides included, are freshened (Bool versus Int). The first equation is not apart from `Cross Bool x`, so it blocks the second. Predicted GHC 9.14.1: rejected (stuck family).
{-# LANGUAGE TypeFamilies #-}
module C03CrossAbstractChirho where

import Data.Proxy (Proxy (..))

type family CrossChirho aChirho bChirho where
  CrossChirho aChirho Int = aChirho
  CrossChirho Bool aChirho = aChirho

crossChirho :: Proxy xChirho -> CrossChirho Bool xChirho -> xChirho
crossChirho _ valueChirho = valueChirho
"###;
    rejected_like_ghc_chirho(source_chirho, "C03CrossAbstractChirho.hs", &["CrossChirho"]);
}

#[test]
fn c08_unshared_stuck_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C08 (tasklist, sharing twin of C07): two different stuck applications are not apart from `Two Int Bool`. Predicted GHC 9.14.1: rejected (stuck family).
{-# LANGUAGE TypeFamilies #-}
module C08UnsharedStuckChirho where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

twoChirho :: Proxy aChirho -> Proxy bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
twoChirho _ _ = 3
"###;
    rejected_like_ghc_chirho(source_chirho, "C08UnsharedStuckChirho.hs", &["TwoChirho"]);
}

#[test]
fn c13_literal_abstract_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control C13 (#30844, tasklist): a literal equation is not apart from an abstract argument, so the fallback must wait for refinement. Predicted GHC 9.14.1: rejected (stuck family).
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures #-}
module C13LiteralAbstractChirho where

import Data.Proxy (Proxy (..))
import GHC.TypeLits (Nat, Symbol)

type family NameChirho (nChirho :: Nat) :: Symbol where
  NameChirho 0 = "zero"
  NameChirho nChirho = "many"

nameChirho :: Proxy nChirho -> Proxy (NameChirho nChirho)
nameChirho _ = (Proxy :: Proxy "many")
"###;
    rejected_like_ghc_chirho(
        source_chirho,
        "C13LiteralAbstractChirho.hs",
        &["NameChirho"],
    );
}
