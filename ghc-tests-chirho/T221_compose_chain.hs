-- TEST: compile_and_run
-- EXPECTED: 33
module Main where
compose f g x = f (g x)
inc x = x + 1; double x = x * 2; square x = x * x
main = print (compose inc (compose double square) 4)
