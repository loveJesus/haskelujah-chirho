-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, PolyKinds, RankNTypes, RequiredTypeArguments, PartialTypeSignatures, TypeOperators #-}
module LateMixedContextLeakChirho where
import GHC.Exts (RuntimeRep(..), TYPE)
data WitnessChirho (rChirho :: RuntimeRep) where
  IntWitnessChirho :: WitnessChirho 'IntRep
badChirho :: forall rChirho sChirho (aChirho :: TYPE rChirho). WitnessChirho sChirho -> _ -> forall bChirho -> (rChirho ~ sChirho) => aChirho
badChirho IntWitnessChirho xChirho bChirho = xChirho
