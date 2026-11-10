-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where

data EmptyChirho qChirho = EmptyChirho (forall aChirho. Ord aChirho => qChirho aChirho)

emptyValuesChirho :: Ord aChirho => [aChirho]
emptyValuesChirho = []
emptyLiteralChirho, emptyAnnotatedChirho, emptyNamedChirho :: EmptyChirho []
emptyLiteralChirho = EmptyChirho []
emptyAnnotatedChirho = EmptyChirho ([] :: Ord aChirho => [aChirho])
emptyNamedChirho = EmptyChirho emptyValuesChirho

isEmptyChirho :: EmptyChirho [] -> Bool
isEmptyChirho (EmptyChirho valuesChirho) = null (valuesChirho :: [Int])

data CompareChirho = CompareChirho (forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool)
compareChirho :: CompareChirho
compareChirho = CompareChirho (\leftChirho rightChirho -> leftChirho < rightChirho)

main :: IO ()
main = do
  print (isEmptyChirho emptyLiteralChirho)
  print (isEmptyChirho emptyAnnotatedChirho)
  print (isEmptyChirho emptyNamedChirho)
  case compareChirho of
    CompareChirho lessChirho -> do
      print (lessChirho (2 :: Int) 3)
      print (lessChirho 'z' 'a')
