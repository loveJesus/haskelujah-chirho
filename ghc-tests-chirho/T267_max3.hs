-- TEST: compile_and_run
-- EXPECTED: 30
module Main where
max' a b = if a > b then a else b
max3 a b c = max' a (max' b c)
main = print (max3 10 30 20)
