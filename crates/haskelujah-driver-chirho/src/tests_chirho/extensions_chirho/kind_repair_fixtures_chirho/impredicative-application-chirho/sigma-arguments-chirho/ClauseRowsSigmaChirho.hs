-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
rowsChirho :: Int -> forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool
rowsChirho 0 leftChirho rightChirho = leftChirho < rightChirho
rowsChirho _ leftChirho rightChirho = rightChirho < leftChirho
lambdaChirho :: Int -> forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool
lambdaChirho = \ignoredChirho leftChirho rightChirho -> leftChirho < rightChirho
main :: IO ()
main = do
  print (rowsChirho 0 (-2.5 :: Double) 1.0, rowsChirho 1 'b' 'a')
  print (lambdaChirho 3 (-2.5 :: Double) 1.0, lambdaChirho 3 'a' 'b')
