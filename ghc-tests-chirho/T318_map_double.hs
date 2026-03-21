-- TEST: compile_and_run
-- EXPECTED: 30
module Main where
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (myMap (*2) [1,2,3,4,5]))
