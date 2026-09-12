-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, PolyKinds #-}
module HigherKindBinderChirho where
import Data.Kind (Type)

data PackageChirho where
  PackageChirho :: forall (containerChirho :: forall kindChirho. kindChirho -> Type).
    containerChirho Int -> containerChirho Maybe -> PackageChirho
