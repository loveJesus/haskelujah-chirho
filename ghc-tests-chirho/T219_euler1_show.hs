-- TEST: compile_and_run
-- EXPECTED: Euler #1: 233168
module Main where
euler1 limit = go 0 0
  where go acc n = if n >= limit then acc else if n `mod` 3 == 0 || n `mod` 5 == 0 then go (acc+n) (n+1) else go acc (n+1)
main = putStrLn ("Euler #1: " ++ show (euler1 1000))
