-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
compareChirho :: Int -> forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool
compareChirho _ leftChirho rightChirho = leftChirho < rightChirho
main :: IO ()
main = print (compareChirho 3 (-2.5 :: Double) 1.0, compareChirho 3 'a' 'b')
