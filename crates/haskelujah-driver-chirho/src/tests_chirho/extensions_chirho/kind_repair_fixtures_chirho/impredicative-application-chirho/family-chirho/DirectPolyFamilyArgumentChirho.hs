-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ImpredicativeTypes #-}
module Main where
type family IdChirho aChirho where
  IdChirho aChirho = aChirho
type BadChirho = IdChirho (forall aChirho. aChirho -> aChirho)
main :: IO ()
main = pure ()
