-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeAbstractions, PolyKinds, CUSKs, KindSignatures #-}
module ClassCuskChirho where
import Data.Kind (Type)
class LicensedChirho @(kChirho :: Type) (aChirho :: kChirho) where
