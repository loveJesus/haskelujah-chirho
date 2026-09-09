-- TEST: compile_and_run
-- EXPECTED: 22
module Main where
myDrop 0 xs = xs; myDrop _ [] = []; myDrop n (_:xs) = myDrop (n-1) xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (myDrop 3 [1,2,3,4,5,6,7]))
