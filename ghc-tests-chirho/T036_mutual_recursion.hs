-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
isEven :: Int -> Int
isEven 0 = 1
isEven n = isOdd (n - 1)
isOdd :: Int -> Int
isOdd 0 = 0
isOdd n = isEven (n - 1)
main :: IO ()
main = do
  print (isEven 10)
  print (isEven 7)
