-- TEST: compile_and_run
-- EXPECTED: 35
module Main where
myReplicate :: Int -> Int -> [Int]
myReplicate 0 _ = []
myReplicate n x = x : myReplicate (n - 1) x
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main = print (mySum (myReplicate 7 5))
