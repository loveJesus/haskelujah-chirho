-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, MagicHash, PolyKinds, TypeFamilies, UnliftedNewtypes #-}
module RecursiveFamilyKindChirho where
import Data.Kind (Type)
import GHC.Exts (TYPE, RuntimeRep(..), Int#)
data TagChirho = MkTagChirho ValueChirho
type family RepChirho (aChirho :: Type) :: RuntimeRep where
  RepChirho TagChirho = 'IntRep
newtype ValueChirho :: TYPE (RepChirho TagChirho) where
  MkValueChirho :: Int# -> ValueChirho
