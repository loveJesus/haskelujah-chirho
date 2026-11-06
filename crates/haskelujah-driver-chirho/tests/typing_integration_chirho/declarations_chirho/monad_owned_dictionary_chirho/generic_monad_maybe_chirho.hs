-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

sevenChirho :: Monad mChirho => mChirho Int
sevenChirho = return 3 >>= \countChirho -> return (countChirho + 4)

doSevenChirho :: Monad mChirho => mChirho Int
doSevenChirho = do
  countChirho <- return 3
  return (countChirho + 4)

bindChirho :: Monad mChirho => mChirho Int -> (Int -> mChirho Int) -> mChirho Int
bindChirho actionChirho nextChirho = actionChirho >>= nextChirho

thenChirho :: Monad mChirho => mChirho Int -> mChirho Int -> mChirho Int
thenChirho firstChirho secondChirho = firstChirho >> secondChirho

main :: IO ()
main = do
  print (sevenChirho :: Maybe Int)
  print (doSevenChirho :: Maybe Int)
  print (sevenChirho :: [Int])
  print (bindChirho (Just 4) (\countChirho -> Just (countChirho + 1)))
  print (bindChirho Nothing (\countChirho -> Just (countChirho + 1)))
  print (thenChirho (Just 1) (Just 2))
  print (thenChirho Nothing (Just 2))
  print (thenChirho [1, 2] [5, 6])
