-- TEST: compile_and_run
-- EXPECTED: 3628800
module Main where
fact 0 = 1; fact n = n * fact (n-1)
main = print (fact 10)
