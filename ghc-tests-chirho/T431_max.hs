-- TEST: compile_and_run
-- EXPECTED: 10
module Main where
myMax [x] = x; myMax (x:xs) = if x > myMax xs then x else myMax xs; myMax [] = 0
main = print (myMax [3,7,2,10,5])
