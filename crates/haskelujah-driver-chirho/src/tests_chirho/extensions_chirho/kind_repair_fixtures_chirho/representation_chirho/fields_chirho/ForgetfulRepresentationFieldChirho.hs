-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures #-}
module ForgetfulRepresentationFieldChirho where
import GHC.Exts (TYPE, RuntimeRep(..))
type IgnoreChirho (rChirho :: RuntimeRep) = 'IntRep
data BoxChirho (rChirho :: RuntimeRep) (aChirho :: TYPE (IgnoreChirho rChirho)) = MkChirho aChirho
