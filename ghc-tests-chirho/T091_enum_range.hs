-- TEST: compile_and_run
-- EXPECTED: 55
module Main where
import Prelude hiding (enumFromTo)
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (enumFromTo 1 10))
