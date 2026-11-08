-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures #-}
module FixedScalarFunctionMethodChirho where
import GHC.Exts (TYPE, RuntimeRep(..), Levity(..))
class FixedChirho (aChirho :: TYPE (IntRep)) where
  fixedChirho :: () -> aChirho
