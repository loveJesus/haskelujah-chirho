-- TEST: compile_and_run
-- EXPECTED: 0\n42\n101
module Main where
f :: [Int] -> Int
f [] = 0
f (x:[]) = x
f (x:_) = x + 100
main :: IO ()
main = do
  print (f [])
  print (f [42])
  print (f [1, 2])
