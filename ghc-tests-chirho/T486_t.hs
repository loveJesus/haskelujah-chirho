-- TEST: compile_and_run
-- EXPECTED: 486
module Main where
id' x = x
main = print (id' 486)
