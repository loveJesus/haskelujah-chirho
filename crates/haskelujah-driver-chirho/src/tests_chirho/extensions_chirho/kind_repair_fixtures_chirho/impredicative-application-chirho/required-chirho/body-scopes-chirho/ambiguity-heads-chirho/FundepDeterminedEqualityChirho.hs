-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
{-# LANGUAGE MultiParamTypeClasses, FunctionalDependencies #-}
module FundepDeterminedEqualityChirho where
import Data.Kind (Type)
import Data.Proxy
class ChooseChirho kChirho aChirho | aChirho -> kChirho
type family KChirho (kChirho :: Type) :: Type
fChirho :: forall (kChirho :: Type). (ChooseChirho kChirho Int, KChirho kChirho ~ Type) => Proxy Int
fChirho = Proxy
