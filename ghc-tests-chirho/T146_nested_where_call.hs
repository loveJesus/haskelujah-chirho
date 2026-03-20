-- TEST: compile_and_run
-- EXPECTED: 41
module Main where
solve :: Int -> Int
solve x = outer x
  where
    outer n = inner (n * 2)
    inner m = m + 1
main :: IO ()
main = print (solve 20)
