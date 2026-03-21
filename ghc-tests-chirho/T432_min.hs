-- TEST: compile_and_run
-- EXPECTED: 1
module Main where
myMin [x] = x; myMin (x:xs) = if x < myMin xs then x else myMin xs; myMin [] = 0
main = print (myMin [3,7,2,10,1,5])
