-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs #-}
module RequiredAliasWrongChirho where
import Data.Proxy
type IdentityChirho = forall aChirho -> aChirho -> aChirho
fChirho :: IdentityChirho
fChirho tChirho (xChirho :: tChirho) = True
