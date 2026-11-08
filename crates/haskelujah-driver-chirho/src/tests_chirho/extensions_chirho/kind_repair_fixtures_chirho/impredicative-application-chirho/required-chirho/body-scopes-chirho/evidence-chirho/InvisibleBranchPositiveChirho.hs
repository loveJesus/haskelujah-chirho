-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module InvisibleBranchPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
data WitnessChirho (aChirho :: kChirho) where
  IntWitnessChirho :: WitnessChirho Int
  TrueWitnessChirho :: WitnessChirho 'True
fChirho :: forall kChirho. forall (aChirho :: kChirho) -> WitnessChirho aChirho -> ()
fChirho tChirho IntWitnessChirho = let { ignoredChirho = (Proxy :: Proxy (tChirho :: Type)) } in ()
fChirho tChirho TrueWitnessChirho = let { ignoredChirho = (Proxy :: Proxy (tChirho :: Bool)) } in ()
