-- TEST: compile_and_run
-- EXPECTED: 120
module Main where
fact 0 = 1; fact n = n * fact (n-1)
main = print (fact 5)
