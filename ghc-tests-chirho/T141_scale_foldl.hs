-- TEST: compile
-- foldl at scale (100 elements for STG step limit)
module Main where
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = print (myFoldl add 0 (enumFromTo 1 100))
