-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
module PlainEqualityGivenChirho where
import Data.Kind (Type)
import Data.Proxy
fChirho :: forall aChirho. (aChirho ~ Int) => aChirho -> aChirho
fChirho xChirho = xChirho
