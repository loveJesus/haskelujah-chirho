-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module Main where
class ClassChirho aChirho where
  data DataChirho aChirho
type family StripChirho aChirho
type instance StripChirho (DataChirho aChirho) = aChirho
keepChirho :: StripChirho (DataChirho Int) -> Int
keepChirho valueChirho = valueChirho
main :: IO ()
main = print (keepChirho 7)
