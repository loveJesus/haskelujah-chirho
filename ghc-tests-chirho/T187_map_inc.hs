-- TEST: compile_and_run
-- EXPECTED: 21
module Main where
myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main = print (mySum (myMap (+1) [1,2,3,4,5]))
