-- TEST: compile_and_run
-- EXPECTED: 20
module Main where
apply f x = f x
inc x = x + 1
double x = x * 2
main = print (apply double (apply inc 9))
