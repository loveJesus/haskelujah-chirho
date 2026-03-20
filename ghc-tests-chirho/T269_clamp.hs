-- TEST: compile_and_run
-- EXPECTED: 0\n50\n100
module Main where
clamp lo hi x | x < lo = lo | x > hi = hi | otherwise = x
main = do { print (clamp 0 100 (-5)); print (clamp 0 100 50); print (clamp 0 100 200) }
