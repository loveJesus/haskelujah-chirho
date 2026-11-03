// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A signature's wanted that nothing in its scope can serve (row 686, control C08): GHC 9.14.1
//! rejects a literal at a family application that never reduces unless a given serves it. Every
//! program is copied byte for byte from the fresh GHC 9.14.1 oracle files of this unit
//! (`tmp-chirho/c08-oracle-chirho/`); C07 and C08 themselves are the family table's.
//! workflow: language-features-chirho/stuck-family-equalities-chirho

use super::{checks_like_ghc_chirho, rejected_like_ghc_chirho};

/// GHC 9.14.1 rejects each with GHC-39999, "No instance for ... arising from the literal 3".
const UNSERVABLE_SIGNATURE_WANTEDS_CHIRHO: [(&str, &str, &str); 3] = [
    (
        "C08LocalProxyChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- C08 with a local Proxy and no import. Predicted GHC 9.14.1: rejected, no instance for Num of the stuck application.
{-# LANGUAGE TypeFamilies #-}
module C08LocalProxyChirho where

data ProxyChirho aChirho = ProxyChirho

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

twoChirho :: ProxyChirho aChirho -> ProxyChirho bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
twoChirho _ _ = 3
"###,
        "TwoChirho",
    ),
    (
        "C08WhereChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- C08's shape as a where-bound signed binding. Predicted GHC 9.14.1: rejected.
{-# LANGUAGE TypeFamilies #-}
module C08WhereChirho where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

outerChirho :: Int
outerChirho = 0
  where
    innerChirho :: Proxy aChirho -> Proxy bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
    innerChirho _ _ = 3
"###,
        "TwoChirho",
    ),
    (
        "C08OpenOnlyChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- A literal at a stuck open family application alone. Predicted GHC 9.14.1: rejected.
{-# LANGUAGE TypeFamilies #-}
module C08OpenOnlyChirho where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

openChirho :: Proxy aChirho -> OpaqueChirho aChirho
openChirho _ = 3
"###,
        "OpaqueChirho",
    ),
];

/// GHC 9.14.1 accepts each: a given or a superclass of one serves the wanted, given equalities
/// let the family reduce, an unsigned binding infers the constraint, and ordinary Num functions
/// and contextual family constraints are untouched.
const SERVED_SIGNATURE_WANTEDS_CHIRHO: [(&str, &str); 6] = [
    (
        "C08GivenChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- C08's shape with the missing constraint given. Predicted GHC 9.14.1: accepted; at Bool and Bool the application reduces to Int, so 3.
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

twoChirho :: Num (TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)) => Proxy aChirho -> Proxy bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
twoChirho _ _ = 3

main :: IO ()
main = print (twoChirho (Proxy :: Proxy Bool) (Proxy :: Proxy Bool))
"###,
    ),
    (
        "C08SuperGivenChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- C08's shape with a superclass of the missing constraint given. Predicted GHC 9.14.1: accepted, 3.
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

twoChirho :: Integral (TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)) => Proxy aChirho -> Proxy bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
twoChirho _ _ = 3

main :: IO ()
main = print (twoChirho (Proxy :: Proxy Bool) (Proxy :: Proxy Bool))
"###,
    ),
    (
        "C08GivenEqChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- C08's shape with given equalities that let the family reduce to Int. Predicted GHC 9.14.1: accepted.
{-# LANGUAGE TypeFamilies #-}
module C08GivenEqChirho where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

twoChirho :: (OpaqueChirho aChirho ~ Int, OpaqueChirho bChirho ~ Char) => Proxy aChirho -> Proxy bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
twoChirho _ _ = 3
"###,
    ),
    (
        "C08UnsignedChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- C08's wanted in an unsigned binding: GHC infers the constraint. Predicted GHC 9.14.1: accepted, 3 at Bool and Bool.
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

useChirho :: Proxy aChirho -> Proxy bChirho -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho) -> TwoChirho (OpaqueChirho aChirho) (OpaqueChirho bChirho)
useChirho _ _ xChirho = xChirho

unsignedChirho pChirho qChirho = useChirho pChirho qChirho 3

main :: IO ()
main = print (unsignedChirho (Proxy :: Proxy Bool) (Proxy :: Proxy Bool))
"###,
    ),
    (
        "NumFunctionsChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Ground, inferred and polymorphic Num functions. Predicted GHC 9.14.1: (4,5,6,4.5).
module Main where

groundChirho :: Int -> Int
groundChirho xChirho = xChirho + 3

inferredChirho xChirho = xChirho + 3

polyChirho :: Num aChirho => aChirho -> aChirho
polyChirho xChirho = xChirho + 3

main :: IO ()
main = print (groundChirho 1, inferredChirho (2 :: Int), polyChirho (3 :: Integer), polyChirho 1.5 :: Double)
"###,
    ),
    (
        "FamilyContextChirho.hs",
        r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Valid contextual family constraints, and a closed family that reduces. Predicted GHC 9.14.1: "42" then 3.
{-# LANGUAGE TypeFamilies #-}
module Main where

import Data.Proxy (Proxy (..))

type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

type family TwoChirho aChirho bChirho where
  TwoChirho Int Bool = Char
  TwoChirho xChirho yChirho = Int

contextChirho :: (Show (OpaqueChirho aChirho), Num (OpaqueChirho aChirho)) => Proxy aChirho -> OpaqueChirho aChirho -> String
contextChirho _ xChirho = show (xChirho + 1)

reducingChirho :: Proxy aChirho -> TwoChirho Int Char
reducingChirho _ = 3

main :: IO ()
main = do
  putStrLn (contextChirho (Proxy :: Proxy Bool) 41)
  print (reducingChirho (Proxy :: Proxy Bool))
"###,
    ),
];

#[test]
fn an_unservable_signature_wanted_is_rejected_like_ghc_chirho() {
    for (file_name_chirho, source_chirho, family_chirho) in UNSERVABLE_SIGNATURE_WANTEDS_CHIRHO {
        rejected_like_ghc_chirho(
            source_chirho,
            file_name_chirho,
            &["no instance for", family_chirho],
        );
    }
}

#[test]
fn a_served_signature_wanted_still_checks_like_ghc_chirho() {
    for (file_name_chirho, source_chirho) in SERVED_SIGNATURE_WANTEDS_CHIRHO {
        checks_like_ghc_chirho(file_name_chirho, source_chirho);
    }
}
