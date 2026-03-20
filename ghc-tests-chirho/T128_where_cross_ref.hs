-- TEST: compile_and_run
-- EXPECTED: 25
module Main where
f :: Int -> Int
f x = result
  where
    result = a + b
    a = x * 2
    b = x * 3
main :: IO ()
main = print (f 5)
