-- TEST: compile_and_run
-- EXPECTED: 476
module Main where
id' x = x
main = print (id' 476)
