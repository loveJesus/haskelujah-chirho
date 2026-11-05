-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs, FlexibleInstances #-}
module RequiredGivenKindPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
data WitnessChirho (kChirho :: Type) where
  TypeWitnessChirho :: WitnessChirho Type
consumeChirho :: forall (aChirho :: Type) -> Int
consumeChirho _ = 0
useChirho :: forall (kChirho :: Type) (aChirho :: kChirho). WitnessChirho kChirho -> Proxy aChirho -> Int
useChirho TypeWitnessChirho _ = consumeChirho aChirho
