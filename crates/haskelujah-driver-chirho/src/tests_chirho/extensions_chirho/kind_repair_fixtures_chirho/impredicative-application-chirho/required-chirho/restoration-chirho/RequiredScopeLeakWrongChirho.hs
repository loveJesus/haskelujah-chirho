-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs, FlexibleInstances #-}
module RequiredScopeLeakWrongChirho where
import Data.Kind (Type)
tChirho :: Bool
tChirho = True
idRequiredChirho :: forall aChirho -> aChirho -> aChirho
idRequiredChirho tChirho (xChirho :: tChirho) = xChirho :: tChirho
consumeChirho :: forall (aChirho :: Type) -> Int
consumeChirho _ = 0
valueChirho :: Int
valueChirho = consumeChirho tChirho
