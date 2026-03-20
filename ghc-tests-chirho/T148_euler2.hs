-- TEST: compile_and_run
-- EXPECTED: 4613732
-- Euler #2: sum of even Fibonacci numbers <= 4M
module Main where
euler2 :: Int -> Int
euler2 limit = go 1 2 0
  where
    go a b acc
      | b > limit = acc
      | b `mod` 2 == 0 = go b (a + b) (acc + b)
      | otherwise = go b (a + b) acc
main :: IO ()
main = print (euler2 4000000)
