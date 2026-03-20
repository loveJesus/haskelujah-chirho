-- TEST: compile_and_run
-- EXPECTED: 66
module Main where
myZip :: [Int] -> [Int] -> [(Int, Int)]
myZip [] _ = []
myZip _ [] = []
myZip (x:xs) (y:ys) = (x, y) : myZip xs ys
sumPairs :: [(Int, Int)] -> Int
sumPairs [] = 0
sumPairs ((a, b):rest) = a + b + sumPairs rest
main :: IO ()
main = print (sumPairs (myZip [1, 2, 3] [10, 20, 30]))
