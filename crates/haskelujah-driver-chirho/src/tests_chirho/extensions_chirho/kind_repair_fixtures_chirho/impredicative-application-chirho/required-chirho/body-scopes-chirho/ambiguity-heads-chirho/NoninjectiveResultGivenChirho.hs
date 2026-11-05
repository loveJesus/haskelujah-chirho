-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module NoninjectiveResultGivenChirho where
import Data.Kind (Type)
import Data.Proxy
type family KChirho (kChirho :: Type) :: Type
fChirho :: forall (kChirho :: Type). (KChirho kChirho ~ Type) => Proxy (KChirho kChirho) -> Int
fChirho _ = 0
