-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module MixedRequiredEtaPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
idChirho :: forall aChirho -> aChirho -> aChirho
idChirho _ xChirho = xChirho
fChirho :: Int -> forall aChirho -> aChirho -> aChirho
fChirho _ = idChirho
