-- TEST: compile_and_run
-- EXPECTED: 41
module Main where
apply f x = f x; double x = x * 2; inc x = x + 1
main = print (apply inc (apply double 20))
