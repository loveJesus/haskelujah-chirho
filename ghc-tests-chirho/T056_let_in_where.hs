-- TEST: compile_and_run
-- EXPECTED: 26
module Main where
f :: Int -> Int
f x = a
  where a = let sq = x * x in sq + 1
main :: IO ()
main = print (f 5)
