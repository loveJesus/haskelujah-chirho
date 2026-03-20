-- TEST: compile_and_run
-- EXPECTED: 55\n120
module Main where
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
myProduct :: [Int] -> Int
myProduct [] = 1
myProduct (x:xs) = x * myProduct xs
main = do { print (mySum [1,2,3,4,5,6,7,8,9,10]); print (myProduct [1,2,3,4,5]) }
