-- TEST: compile_and_run
-- EXPECTED: 6
module Main where
myTake 0 _ = []; myTake _ [] = []; myTake n (x:xs) = x : myTake (n-1) xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (myTake 3 [1,2,3,4,5]))
