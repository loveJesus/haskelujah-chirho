-- TEST: compile_and_run
-- EXPECTED: 248
module Main where
id' x = x
main = print (id' 248)
