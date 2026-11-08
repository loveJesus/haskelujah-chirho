-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures, TypeFamilies #-}
module ClosedFamilyFieldChirho where
import GHC.Exts (TYPE, RuntimeRep(..))
type family RepChirho :: RuntimeRep where RepChirho = 'IntRep
data BoxChirho (aChirho :: TYPE RepChirho) = MkChirho aChirho
