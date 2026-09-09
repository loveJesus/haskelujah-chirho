-- TEST: compile_and_run
-- EXPECTED: 220
module Main where
import Prelude hiding (enumFromTo)
myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
main :: IO ()
main = print (mySum (myMap (\x -> x * x) (myFilter (\x -> x `mod` 2 == 0) (enumFromTo 1 10))))
