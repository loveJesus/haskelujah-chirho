-- TEST: compile_and_run
-- EXPECTED: 55
module Main where
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum [1,2,3,4,5,6,7,8,9,10])
