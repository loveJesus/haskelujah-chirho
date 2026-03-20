-- TEST: compile_and_run
-- EXPECTED: 32
module Main where
myZip [] _ = []; myZip _ [] = []; myZip (x:xs) (y:ys) = (x,y) : myZip xs ys
pairSum :: [(Int,Int)] -> Int
pairSum [] = 0; pairSum ((a,b):rest) = a + b + pairSum rest
main = print (pairSum (myZip [1,2,3] [4,5,6,7]))
