-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
myElem _ [] = False; myElem x (y:ys) = if x == y then True else myElem x ys
main = do { print (bti (myElem 3 [1,2,3,4])); print (bti (myElem 5 [1,2,3,4])) }
