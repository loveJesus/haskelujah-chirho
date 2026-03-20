-- TEST: compile_and_run
-- EXPECTED: 111
module Main where
collatz :: Int -> Int
collatz n = go n 0
  where
    go 1 steps = steps
    go n steps
      | n `mod` 2 == 0 = go (n `div` 2) (steps + 1)
      | otherwise = go (3 * n + 1) (steps + 1)
main :: IO ()
main = print (collatz 27)
