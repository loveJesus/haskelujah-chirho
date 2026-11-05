-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs, FlexibleInstances #-}
module RequiredMethodWrongChirho where
import Data.Kind (Type)
import Data.Proxy
consumeChirho :: forall (fChirho :: Type -> Type) -> Int
consumeChirho _ = 0
class CChirho aChirho where
  methodChirho :: forall (fChirho :: Type -> Type) -> Proxy fChirho -> aChirho -> Int
  methodChirho tChirho (_ :: Proxy tChirho) _ = consumeChirho Int
instance CChirho Int
valueChirho :: Int
valueChirho = methodChirho Maybe Proxy (3 :: Int)
