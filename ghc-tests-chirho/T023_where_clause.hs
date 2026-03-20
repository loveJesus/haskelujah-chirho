-- TEST: compile_and_run
-- EXPECTED: 5050
module Main where
sumTo :: Int -> Int
sumTo n = go 0 0
  where go acc i = if i > n then acc else go (acc + i) (i + 1)
main :: IO ()
main = print (sumTo 100)
