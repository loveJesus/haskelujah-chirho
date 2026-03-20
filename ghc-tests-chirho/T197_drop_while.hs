-- TEST: compile_and_run
-- EXPECTED: 18
module Main where
myDropWhile :: (Int -> Bool) -> [Int] -> [Int]
myDropWhile _ [] = []
myDropWhile p (x:xs) = if p x then myDropWhile p xs else x : xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main = print (mySum (myDropWhile (\x -> x < 5) [1,2,3,4,5,6,7]))
