-- TEST: compile_and_run
-- EXPECTED: 220
module Main where
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myFoldl _ a [] = a; myFoldl f a (x:xs) = myFoldl f (f a x) xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
add a b = a + b
main = print (myFoldl add 0 (myMap (\x -> x*x) (myFilter (\x -> x `mod` 2 == 0) (enumFromTo 1 10))))
