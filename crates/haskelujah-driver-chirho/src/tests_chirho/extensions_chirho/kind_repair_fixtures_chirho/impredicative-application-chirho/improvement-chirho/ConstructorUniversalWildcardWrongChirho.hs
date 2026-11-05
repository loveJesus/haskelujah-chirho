-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, PolyKinds, TypeApplications, TypeAbstractions, ScopedTypeVariables #-}
module ConstructorUniversalWildcardWrongChirho where
import Data.Kind (Type)
data TChirho where
  MkTChirho :: forall (fChirho :: forall kChirho. kChirho -> Type). fChirho Int -> fChirho Maybe -> TChirho
gChirho :: TChirho -> ()
gChirho (MkTChirho @(_ :: Type) _ _) = ()
