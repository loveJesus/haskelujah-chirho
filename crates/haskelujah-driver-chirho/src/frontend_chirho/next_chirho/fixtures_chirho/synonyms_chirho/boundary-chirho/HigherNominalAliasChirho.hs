-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module HigherNominalAliasChirho where
data BoxChirho aChirho = BoxChirho aChirho
data HigherChirho fChirho = HigherChirho (fChirho Bool)
type AliasChirho = HigherChirho BoxChirho
