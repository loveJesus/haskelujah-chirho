-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables #-}
module RequiredDependentPositiveChirho where
import Data.Kind (Type)
consumeChirho :: forall (kChirho :: Type) -> forall (aChirho :: kChirho) -> Int
consumeChirho _ _ = 0
valueChirho :: Int
valueChirho = consumeChirho Type Int
