-- TEST: compile_and_run
-- EXPECTED: 466
module Main where
id' x = x
main = print (id' 466)
