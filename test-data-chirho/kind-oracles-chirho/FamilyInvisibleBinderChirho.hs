-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeAbstractions, TypeFamilies, CUSKs #-}
module FamilyInvisibleBinderChirho where
import Data.Kind (Type)

type family FamilyChirho @(kChirho :: Type) (aChirho :: kChirho) :: kChirho where
  FamilyChirho aChirho = aChirho
