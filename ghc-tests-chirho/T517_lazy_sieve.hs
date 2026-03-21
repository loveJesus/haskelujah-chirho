-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [2,3,5,7,11,13,17,19,23,29]
module Main where
myFilter :: (a -> Bool) -> [a] -> [a]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
from :: Int -> [Int]
from n = n : from (n + 1)
sieve :: [Int] -> [Int]
sieve (p:xs) = p : sieve (myFilter (\x -> mod x p /= 0) xs)
primes :: [Int]
primes = sieve (from 2)
main = print (myTake 10 primes)
