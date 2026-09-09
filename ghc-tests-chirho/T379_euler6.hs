-- TEST: compile_and_run
-- EXPECTED: 25164150
module Main where
sumTo n = n * (n + 1) `div` 2
sumSq n = go 1 0 where go i acc = if i > n then acc else go (i+1) (acc + i*i)
main = do { let { s = sumTo 100 }; print (s * s - sumSq 100) }
