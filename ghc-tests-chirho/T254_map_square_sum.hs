-- TEST: compile_and_run
-- EXPECTED: 55
module Main where
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (myMap (\x -> x * x) [1,2,3,4,5]))
