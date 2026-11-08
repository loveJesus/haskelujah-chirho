-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module ConstantFamilyKindGivenChirho where
import Data.Kind (Type)
import Data.Proxy
type family KChirho (kChirho :: Type) :: Type where { KChirho kChirho = Type }
fChirho :: forall (kChirho :: Type) (aChirho :: KChirho kChirho). (KChirho kChirho ~ Type) => Proxy aChirho -> Int
fChirho _ = 0
