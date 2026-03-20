-- TEST: compile_and_run
-- EXPECTED: 75
module Main where
myZipWith :: (Int -> Int -> Int) -> [Int] -> [Int] -> [Int]
myZipWith _ [] _ = []
myZipWith _ _ [] = []
myZipWith f (x:xs) (y:ys) = f x y : myZipWith f xs ys
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = print (mySum (myZipWith add (enumFromTo 1 5) (enumFromTo 10 14)))
