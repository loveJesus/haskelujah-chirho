-- TEST: compile_and_run
-- EXPECTED: 111
module Main where
collatz n = go n 0
  where go 1 s = s; go n s | n `mod` 2 == 0 = go (n `div` 2) (s+1) | otherwise = go (3*n+1) (s+1)
main = print (collatz 27)
