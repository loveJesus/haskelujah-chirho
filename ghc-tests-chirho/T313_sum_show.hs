-- TEST: compile_and_run
-- EXPECTED: sum = 55
module Main where
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = putStrLn ("sum = " ++ show (mySum [1,2,3,4,5,6,7,8,9,10]))
