-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs #-}
module RequiredQualPositiveChirho where
import Data.Proxy
fChirho :: forall aChirho -> Eq aChirho => aChirho -> Bool
fChirho tChirho xChirho = xChirho == xChirho
