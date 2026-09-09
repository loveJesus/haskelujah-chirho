-- TEST: compile_and_run
-- EXPECTED: 5
module Main where
import Prelude hiding (enumFromTo)
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
main = print (myLen (myFilter (\x -> x `mod` 2 == 0) (enumFromTo 1 10)))
