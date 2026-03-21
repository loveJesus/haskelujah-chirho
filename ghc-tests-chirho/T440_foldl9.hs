-- TEST: compile_and_run
-- EXPECTED: 45
module Main where
myFoldl _ a [] = a; myFoldl f a (x:xs) = myFoldl f (f a x) xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
add a b = a + b
main = print (myFoldl add 0 (enumFromTo 1 9))
