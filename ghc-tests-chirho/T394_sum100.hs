-- TEST: compile_and_run
-- EXPECTED: 5050
module Main where
sumTo n = go 0 1 where go a i = if i > n then a else go (a+i) (i+1)
main = print (sumTo 100)
