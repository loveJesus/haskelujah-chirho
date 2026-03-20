-- TEST: compile_and_run
-- EXPECTED: 1\n0\n1
module Main where
bti :: Bool -> Int
bti True = 1
bti False = 0
main = do { print (bti (4 `mod` 2 == 0)); print (bti (7 `mod` 2 == 0)); print (bti (100 `mod` 2 == 0)) }
