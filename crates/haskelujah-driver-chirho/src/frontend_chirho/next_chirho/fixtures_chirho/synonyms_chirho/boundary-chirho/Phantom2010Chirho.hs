-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module Phantom2010Chirho where
data BoxChirho aChirho = BoxChirho aChirho
type EraseChirho aChirho = Bool
resultChirho :: EraseChirho BoxChirho
resultChirho = True
