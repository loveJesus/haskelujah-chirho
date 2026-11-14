-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module HigherField2010Chirho where
data UseChirho = UseChirho (AliasChirho BoxChirho)
type AliasChirho aChirho = HigherChirho aChirho
data HigherChirho fChirho = HigherChirho (fChirho Bool)
data BoxChirho aChirho = BoxChirho aChirho
