-- TEST: compile_and_run
-- EXPECTED: 5050
module Main where
sumTo :: Int -> Int
sumTo 0 = 0
sumTo n = n + sumTo (n - 1)
main = print (sumTo 100)
