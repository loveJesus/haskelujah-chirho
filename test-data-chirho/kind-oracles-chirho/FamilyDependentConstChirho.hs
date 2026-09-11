-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021, DataKinds, TypeFamilies #-}
module FamilyDependentConstChirho where
import Data.Kind (Type)

type ConstChirho :: aChirho -> bChirho -> aChirho
type family ConstChirho xChirho yChirho where
  ConstChirho xChirho _ = xChirho

type ConsumerChirho :: (forall (bChirho :: Bool) -> ConstChirho Type bChirho) -> Type
data ConsumerChirho fChirho
type ArgumentChirho :: forall (bChirho :: Bool) -> Type
data ArgumentChirho bChirho
type family ResultChirho where
  ResultChirho = ConsumerChirho ArgumentChirho
