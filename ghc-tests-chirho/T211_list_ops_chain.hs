-- TEST: compile_and_run
-- EXPECTED: 60
module Main where
myMap f [] = []; myMap f (x:xs) = f x : myMap f xs
myFilter p [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (myMap (*2) (myFilter (\x -> x `mod` 2 == 0) [1,2,3,4,5,6,7,8,9,10])))
