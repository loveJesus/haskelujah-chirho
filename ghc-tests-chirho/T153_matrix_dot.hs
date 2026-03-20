-- TEST: compile_and_run
-- EXPECTED: 460
module Main where
dot :: [Int] -> [Int] -> Int
dot [] _ = 0
dot _ [] = 0
dot (x:xs) (y:ys) = x * y + dot xs ys
matVec :: [[Int]] -> [Int] -> [Int]
matVec [] _ = []
matVec (row:rows) v = dot row v : matVec rows v
vecSum :: [Int] -> Int
vecSum [] = 0
vecSum (x:xs) = x + vecSum xs
main :: IO ()
main = print (vecSum (matVec [[1,2,3],[4,5,6]] [10,20,30]))
