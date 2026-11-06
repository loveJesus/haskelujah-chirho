-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures, TypeFamilies #-}
module OriginChirho (BoxChirho) where
import GHC.Exts (TYPE, RuntimeRep(..))
data BoxChirho (rChirho :: RuntimeRep) (aChirho :: TYPE rChirho) = MkChirho aChirho
