-- TEST: compile_and_run
-- EXPECTED: 456
module Main where
id' x = x
main = print (id' 456)
