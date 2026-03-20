-- TEST: compile_and_run
-- EXPECTED: 10
module Main where
myTakeWhile :: (Int -> Bool) -> [Int] -> [Int]
myTakeWhile _ [] = []
myTakeWhile p (x:xs) = if p x then x : myTakeWhile p xs else []
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main = print (mySum (myTakeWhile (\x -> x < 5) [1,2,3,4,5,6,7]))
