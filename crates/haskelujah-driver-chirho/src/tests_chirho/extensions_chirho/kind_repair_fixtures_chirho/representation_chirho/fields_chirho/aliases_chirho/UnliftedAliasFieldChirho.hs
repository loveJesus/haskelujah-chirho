-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures #-}
module UnliftedAliasFieldChirho where
import GHC.Exts (UnliftedType)
data BoxChirho (aChirho :: UnliftedType) = MkChirho aChirho
