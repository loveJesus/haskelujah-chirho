-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module MixedContextPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
hChirho :: Int -> forall aChirho -> Eq aChirho => aChirho -> aChirho -> Bool
hChirho _ tChirho xChirho yChirho = xChirho == yChirho
