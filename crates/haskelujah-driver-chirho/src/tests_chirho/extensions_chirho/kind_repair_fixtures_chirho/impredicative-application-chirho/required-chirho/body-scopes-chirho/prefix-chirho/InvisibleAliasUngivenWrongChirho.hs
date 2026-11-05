-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module InvisibleAliasUngivenWrongChirho where
import Data.Kind (Type)
import Data.Proxy
data WitnessChirho (kChirho :: Type) where
  TypeWitnessChirho :: WitnessChirho Type
type SignatureChirho = forall (kChirho :: Type). forall (aChirho :: kChirho) -> WitnessChirho kChirho -> Proxy aChirho
fChirho :: SignatureChirho
fChirho tChirho _ = (Proxy :: Proxy (tChirho :: Type))
