-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures, ScopedTypeVariables, ExplicitForAll, FlexibleInstances #-}
module PolykindInstanceContextChirho where
import Data.Kind (Type)
data WitnessChirho (aChirho :: kChirho) = WitnessChirho
class DescribeChirho (aChirho :: kChirho) where
  describeChirho :: WitnessChirho aChirho -> Int
data WrapChirho (aChirho :: Type) = WrapChirho
instance forall aChirho. DescribeChirho aChirho => Show (WrapChirho aChirho) where
  show _ = show (describeChirho (WitnessChirho :: WitnessChirho aChirho))
