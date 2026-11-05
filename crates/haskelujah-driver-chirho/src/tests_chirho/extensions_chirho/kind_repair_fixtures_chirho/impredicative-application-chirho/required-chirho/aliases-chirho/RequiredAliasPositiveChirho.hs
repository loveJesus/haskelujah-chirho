-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs #-}
module RequiredAliasPositiveChirho where
import Data.Proxy
type IdentityChirho = forall aChirho -> aChirho -> aChirho
fChirho :: IdentityChirho
fChirho tChirho (xChirho :: tChirho) = xChirho :: tChirho
valueChirho = fChirho Int (3 :: Int)
