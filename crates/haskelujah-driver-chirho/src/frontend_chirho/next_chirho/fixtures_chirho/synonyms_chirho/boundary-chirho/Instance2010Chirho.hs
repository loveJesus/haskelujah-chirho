-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module Instance2010Chirho where
class HomeChirho aChirho where tagChirho :: aChirho -> Char
data BoxChirho = BoxChirho
type AliasChirho = BoxChirho
instance HomeChirho AliasChirho where tagChirho _ = 'i'
resultChirho = tagChirho BoxChirho
