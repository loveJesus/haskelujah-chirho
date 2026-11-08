-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures, TypeFamilies #-}
module OrdinaryRecursiveFamilyChirho where
import Data.Kind (Type)
data TagChirho = MkTagChirho (LinkChirho TagChirho)
type family LinkChirho (aChirho :: Type) :: Type where
  LinkChirho TagChirho = Int
