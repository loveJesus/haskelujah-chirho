-- TEST: compile_and_run
-- EXPECTED: 21
module Main where
compose f g x = f (g x)
inc x = x + 1; double x = x * 2; addTen x = x + 10
pipeline = compose inc (compose double addTen)
main = print (pipeline 0)
