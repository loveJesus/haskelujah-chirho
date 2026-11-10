-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
data RecordChirho = RecordChirho { compareFieldChirho :: forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool }
recordChirho :: RecordChirho
recordChirho = RecordChirho { compareFieldChirho = \leftChirho rightChirho -> leftChirho < rightChirho }
lambdaResultChirho :: Int -> (forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool)
lambdaResultChirho = \ignoredChirho -> \leftChirho rightChirho -> leftChirho < rightChirho
clauseResultChirho :: Int -> (forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool)
clauseResultChirho ignoredChirho = \leftChirho rightChirho -> leftChirho < rightChirho
main :: IO ()
main = do
  case recordChirho of
    RecordChirho lessChirho -> print (lessChirho (2 :: Int) 3)
  print (compareFieldChirho recordChirho (-2.5 :: Double) 1.0)
  print (lambdaResultChirho 3 (-2.5 :: Double) 1.0)
  print (clauseResultChirho 3 'a' 'b')
