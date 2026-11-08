-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, MagicHash, PolyKinds, StandaloneKindSignatures, TypeFamilies, UnliftedNewtypes #-}
module RecursiveCheckedFamilyKindChirho where
import Data.Kind (Type)
import GHC.Exts (TYPE, RuntimeRep(..), Int#)
type TagChirho :: Type
data TagChirho = MkTagChirho ValueChirho
type RepChirho :: Type -> RuntimeRep
type family RepChirho aChirho where
  RepChirho TagChirho = 'IntRep
type ValueChirho :: TYPE (RepChirho TagChirho)
newtype ValueChirho where
  MkValueChirho :: Int# -> ValueChirho
