-- TEST: compile_and_run
-- EXPECTED: 238
module Main where
id' x = x
main = print (id' 238)
