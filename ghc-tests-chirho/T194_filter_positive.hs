-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myLen :: [Int] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = print (myLen (myFilter (\x -> x > 0) [-2, 3, -1, 5, 0, 7]))
