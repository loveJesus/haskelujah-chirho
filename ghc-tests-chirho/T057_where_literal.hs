-- TEST: compile_and_run
-- EXPECTED: 50
module Main where
f :: Int -> Int
f x = y + x
  where y = 42
main :: IO ()
main = print (f 8)
