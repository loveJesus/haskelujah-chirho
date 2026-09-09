-- TEST: compile_and_run
-- EXPECTED: 12586269025\n111\n21\n25
-- Comprehensive test: fibonacci, collatz, GCD, primes
module Main where
import Prelude hiding (enumFromTo)
fib :: Int -> Int
fib n = go n 0 1
  where go 0 a _ = a
        go n a b = go (n - 1) b (a + b)
collatz :: Int -> Int
collatz n = go n 0
  where
    go 1 steps = steps
    go n steps
      | n `mod` 2 == 0 = go (n `div` 2) (steps + 1)
      | otherwise = go (3 * n + 1) (steps + 1)
gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
isPrime :: Int -> Bool
isPrime n
  | n < 2 = False
  | otherwise = go 2
  where go d
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
main = do
  print (fib 50)
  print (collatz 27)
  print (gcd' 252 105)
  print (myLength (myFilter isPrime (enumFromTo 2 100)))
