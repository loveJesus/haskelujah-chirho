-- TEST: compile_and_run
-- EXPECTED: 75
module Main where
myZipWith _ [] _ = []; myZipWith _ _ [] = []; myZipWith f (x:xs) (y:ys) = f x y : myZipWith f xs ys
mySum [] = 0; mySum (x:xs) = x + mySum xs
add a b = a + b
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
main = print (mySum (myZipWith add (enumFromTo 1 5) (enumFromTo 10 14)))
