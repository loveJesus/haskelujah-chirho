-- TEST: compile_and_run
-- EXPECTED: 120
module Main where
half x = x `div` 2
main = print (half 240)
