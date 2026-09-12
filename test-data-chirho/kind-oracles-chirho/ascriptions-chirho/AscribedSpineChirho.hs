-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, PolyKinds, StandaloneKindSignatures, TypeApplications, TypeFamilies #-}
module Main where
import Data.Kind (Type)

data ProxyChirho (valueChirho :: keyChirho) = ProxyChirho
type IdentityChirho (valueChirho :: keyChirho) = valueChirho
type ClassifierChirho :: forall keyChirho. keyChirho -> Type
type family ClassifierChirho valueChirho where
  ClassifierChirho (valueChirho :: Type) = Int
  ClassifierChirho (valueChirho :: Bool) = Bool
type family IgnoreBothChirho (leftChirho :: Type) (rightChirho :: Type) :: Type where
  IgnoreBothChirho _ _ = Bool

data EqualChirho (leftChirho :: keyChirho) (rightChirho :: keyChirho) where
  ReflChirho :: EqualChirho valueChirho valueChirho

nominalChirho :: EqualChirho ((ProxyChirho :: Bool -> Type) 'True) (ProxyChirho 'True)
nominalChirho = ReflChirho
wholeChirho :: EqualChirho (ProxyChirho 'True :: Type) (ProxyChirho 'True)
wholeChirho = ReflChirho
aliasChirho :: EqualChirho ((IdentityChirho :: Bool -> Bool) 'True) 'True
aliasChirho = ReflChirho
familyChirho :: EqualChirho ((ClassifierChirho :: Bool -> Type) 'True) Bool
familyChirho = ReflChirho
anonymousChirho :: EqualChirho (IgnoreBothChirho Int Bool) Bool
anonymousChirho = ReflChirho

main :: IO ()
main = case nominalChirho of
  ReflChirho -> case wholeChirho of
    ReflChirho -> case aliasChirho of
      ReflChirho -> case familyChirho of
        ReflChirho -> case anonymousChirho of
          ReflChirho -> putStrLn "ascribed spines"
