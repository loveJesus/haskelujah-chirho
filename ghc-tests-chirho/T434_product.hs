-- TEST: compile_and_run
-- EXPECTED: 120
module Main where
myProduct [] = 1; myProduct (x:xs) = x * myProduct xs
main = print (myProduct [1,2,3,4,5])
