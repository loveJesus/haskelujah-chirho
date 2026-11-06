-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- A lambda-valued simple pattern binding without a signature is restricted: one type only.
plusOneChirho = \valueChirho -> valueChirho + 1

main :: IO ()
main = print (plusOneChirho (1 :: Int), plusOneChirho (2.5 :: Double))
