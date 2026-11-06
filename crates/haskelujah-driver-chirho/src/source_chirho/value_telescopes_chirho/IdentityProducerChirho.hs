-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ExplicitForAll, KindSignatures #-}
module IdentityProducerChirho (id) where
import Prelude hiding (id)
import Data.Kind (Type)
id :: forall (fChirho :: Type -> Type). fChirho Int -> fChirho Int
id xChirho = xChirho
