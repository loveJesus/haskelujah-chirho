-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module ImportedTypePatternChirho where
import qualified ProviderChirho as PChirho
data family TypeChirho aChirho
type family StripChirho aChirho
type instance StripChirho (PChirho.TypeChirho aChirho) = aChirho
