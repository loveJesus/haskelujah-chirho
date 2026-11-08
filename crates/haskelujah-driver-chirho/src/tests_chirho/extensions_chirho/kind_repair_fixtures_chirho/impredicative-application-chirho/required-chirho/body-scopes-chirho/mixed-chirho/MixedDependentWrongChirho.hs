-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module MixedDependentWrongChirho where
import Data.Kind (Type)
import Data.Proxy
fChirho :: Bool -> forall (kChirho :: Type) -> Int -> forall (aChirho :: kChirho) -> Proxy aChirho -> Proxy aChirho
fChirho _ kChirho _ aChirho (xChirho :: Proxy aChirho) = Proxy :: Proxy (aChirho :: Type)
