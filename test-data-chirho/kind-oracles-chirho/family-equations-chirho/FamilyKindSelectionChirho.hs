-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeApplications #-}
module Main where
import Data.Kind (Type)
data FlagChirho (valueChirho :: Bool) = FlagChirho Int
type family PickChirho :: kindChirho -> Type
type instance PickChirho = Maybe
type instance PickChirho = FlagChirho
ordinaryChirho :: PickChirho @Type Int
ordinaryChirho = Just 42
promotedChirho :: PickChirho @Bool 'True
promotedChirho = FlagChirho 7
main = case ordinaryChirho of
  Just firstChirho -> case promotedChirho of
    FlagChirho secondChirho -> print (firstChirho + secondChirho)
