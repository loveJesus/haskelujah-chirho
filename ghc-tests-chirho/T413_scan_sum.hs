-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
myScanl _ a [] = [a]; myScanl f a (x:xs) = a : myScanl f (f a x) xs
myLast [x] = x; myLast (_:xs) = myLast xs; myLast [] = 0
add a b = a + b
main = print (myLast (myScanl add 0 [1,2,3,4,5]))
