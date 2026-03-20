-- TEST: compile_and_run
-- EXPECTED: 25164150
-- Euler #6: sum-square difference for 1..100
module Main where
sumTo :: Int -> Int
sumTo n = n * (n + 1) `div` 2
sumSq :: Int -> Int
sumSq n = go 1 0
  where go i acc = if i > n then acc else go (i + 1) (acc + i * i)
main :: IO ()
main = do
  let s = sumTo 100
  let sq = sumSq 100
  print (s * s - sq)
