-- TEST: compile_and_run
-- EXPECTED: 10
module Main where
min' a b = if a < b then a else b
min3 a b c = min' a (min' b c)
main = print (min3 10 30 20)
