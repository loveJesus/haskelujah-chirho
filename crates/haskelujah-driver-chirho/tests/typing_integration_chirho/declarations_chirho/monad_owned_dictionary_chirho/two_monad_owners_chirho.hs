-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

chainBothChirho :: (Monad mChirho, Monad nChirho) => mChirho Int -> mChirho Int -> nChirho Int -> nChirho Int -> (mChirho Int, nChirho Int)
chainBothChirho firstMChirho secondMChirho firstNChirho secondNChirho =
  (firstMChirho >> secondMChirho, firstNChirho >> secondNChirho)

main :: IO ()
main = print (chainBothChirho (Just 1) (Just 2) [3] [4, 5])
