-- TEST: compile_and_run
-- EXPECTED: 65
module Main where
import Prelude hiding (enumFromTo)
myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
add :: Int -> Int -> Int
add x y = x + y
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
main :: IO ()
main = print (mySum (myMap (add 10) (enumFromTo 1 5)))
