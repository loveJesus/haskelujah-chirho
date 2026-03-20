-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
myNull [] = True; myNull _ = False
main = do { print (bti (myNull [])); print (bti (myNull [1])) }
