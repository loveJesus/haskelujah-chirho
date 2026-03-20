-- TEST: compile_and_run
-- EXPECTED: 7\n-1
module Main where
myAbs :: Int -> Int
myAbs n = if n >= 0 then n else 0 - n
mySignum :: Int -> Int
mySignum n
  | n > 0 = 1
  | n == 0 = 0
  | otherwise = -1
main :: IO ()
main = do
  print (myAbs (-7))
  print (mySignum (-42))
