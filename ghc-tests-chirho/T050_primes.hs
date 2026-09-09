-- TEST: compile_and_run
-- EXPECTED: 25
module Main where
import Prelude hiding (enumFromTo)
isPrime :: Int -> Bool
isPrime n
  | n < 2 = False
  | otherwise = go 2
  where
    go d
      | d * d > n = True
      | n `mod` d == 0 = False
      | otherwise = go (d + 1)
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
main :: IO ()
main = print (myLength (myFilter isPrime (enumFromTo 2 100)))
