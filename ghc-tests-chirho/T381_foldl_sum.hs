-- TEST: compile_and_run
-- EXPECTED: 55
module Main where
myFoldl _ a [] = a; myFoldl f a (x:xs) = myFoldl f (f a x) xs
add a b = a + b
main = print (myFoldl add 0 [1,2,3,4,5,6,7,8,9,10])
