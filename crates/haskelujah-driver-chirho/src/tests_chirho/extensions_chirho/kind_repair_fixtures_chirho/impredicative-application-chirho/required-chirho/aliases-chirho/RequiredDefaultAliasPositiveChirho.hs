-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs #-}
module RequiredDefaultAliasPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
consumeChirho :: forall (fChirho :: Type -> Type) -> Int
consumeChirho _ = 0
type MethodChirho aChirho = forall (fChirho :: Type -> Type) -> Proxy fChirho -> aChirho -> Int
class CChirho aChirho where
  methodChirho :: MethodChirho aChirho
  methodChirho tChirho (_ :: Proxy tChirho) _ = consumeChirho tChirho
