-- TEST: compile_and_run
-- EXPECTED: 65
module Main where
import Prelude hiding (enumFromTo)
myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
add :: Int -> Int -> Int
add x y = x + y
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
main :: IO ()
main = print (myFoldl add 0 (myMap (add 10) (enumFromTo 1 5)))
