-- TEST: compile_and_run
-- EXPECTED: 25
module Main where
import Prelude hiding (enumFromTo)
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
isPrime n | n < 2 = False | otherwise = go 2 where go d | d*d > n = True | n `mod` d == 0 = False | otherwise = go (d+1)
main = print (myLen (myFilter isPrime (enumFromTo 2 100)))
