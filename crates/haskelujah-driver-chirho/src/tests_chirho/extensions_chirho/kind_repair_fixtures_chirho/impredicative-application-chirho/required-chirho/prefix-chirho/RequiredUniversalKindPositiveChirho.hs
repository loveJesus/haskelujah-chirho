-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs #-}
module RequiredUniversalKindPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
fChirho :: forall (kChirho :: Type) -> forall (aChirho :: kChirho) -> Proxy aChirho
fChirho kChirho aChirho = (Proxy :: Proxy aChirho)
