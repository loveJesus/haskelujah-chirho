-- TEST: compile_and_run
-- EXPECTED: 1024
module Main where
pow' _ 0 = 1; pow' b e = b * pow' b (e-1)
main = print (pow' 2 10)
