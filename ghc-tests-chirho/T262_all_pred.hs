-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
myAll _ [] = True; myAll p (x:xs) = if p x then myAll p xs else False
main = do { print (bti (myAll (\x -> x > 0) [1,2,3])); print (bti (myAll (\x -> x > 0) [1,-2,3])) }
