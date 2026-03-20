-- TEST: compile_and_run
-- EXPECTED: 80
module Main where
compose f g x = f (g x)
double x = x * 2
quadruple = compose double double
main = print (quadruple 20)
