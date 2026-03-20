-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (append [1, 2, 3] [4, 5]))
