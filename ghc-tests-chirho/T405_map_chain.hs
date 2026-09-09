-- TEST: compile_and_run
-- EXPECTED: 220
module Main where
import Prelude hiding (enumFromTo)
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
main = print (mySum (myMap (\x -> x*x) (myFilter (\x -> x `mod` 2 == 0) (enumFromTo 1 10))))
