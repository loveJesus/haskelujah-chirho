-- TEST: compile_and_run
-- EXPECTED: 385
module Main where
sumSquares n = go 1 0
  where go i acc = if i > n then acc else go (i+1) (acc + i*i)
main = print (sumSquares 10)
