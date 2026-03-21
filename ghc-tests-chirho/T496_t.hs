-- TEST: compile_and_run
-- EXPECTED: 496
module Main where
id' x = x
main = print (id' 496)
