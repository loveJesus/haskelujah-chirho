-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- A non-simple pattern binding stays restricted even when every binder has a signature.
leftChirho :: Num a => a
rightChirho :: Num a => a
(leftChirho, rightChirho) = (1, 2)

main :: IO ()
main = print (leftChirho :: Int, rightChirho :: Double)
