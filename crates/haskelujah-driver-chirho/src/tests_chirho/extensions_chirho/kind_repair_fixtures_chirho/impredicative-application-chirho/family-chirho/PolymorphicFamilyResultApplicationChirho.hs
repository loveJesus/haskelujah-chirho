-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ImpredicativeTypes, StandaloneKindSignatures #-}
module Main where
import Data.Kind (Type)
type MakeChirho :: Type -> Type
type family MakeChirho where
  MakeChirho = Maybe
witnessChirho :: MakeChirho (forall aChirho. aChirho -> aChirho)
witnessChirho = Nothing
main :: IO ()
main = pure ()
