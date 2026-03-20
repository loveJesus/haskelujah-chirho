-- TEST: compile_and_run
-- EXPECTED: 287
module Main where
id' x = x
main = print (id' 287)
