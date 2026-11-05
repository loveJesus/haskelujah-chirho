-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures, GADTs, RankNTypes, TypeOperators #-}
module RefinedRepresentationFieldChirho where
import GHC.Exts (TYPE, RuntimeRep(..))
data BoxChirho (rChirho :: RuntimeRep) (aChirho :: TYPE rChirho) where
  MkChirho :: forall (rChirho :: RuntimeRep) (aChirho :: TYPE rChirho). (rChirho ~ 'IntRep) => aChirho -> BoxChirho rChirho aChirho
