-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, PolyKinds, KindSignatures, RankNTypes, ScopedTypeVariables, MagicHash #-}
module WildcardValueParameterChirho where
import GHC.Exts (RuntimeRep(..), TYPE)
data WitnessChirho (rChirho :: RuntimeRep) where
  IntWitnessChirho :: WitnessChirho 'IntRep
  DoubleWitnessChirho :: WitnessChirho 'DoubleRep
valueChirho :: forall (rChirho :: RuntimeRep) (aChirho :: TYPE rChirho). aChirho -> Int
valueChirho _ = 0
