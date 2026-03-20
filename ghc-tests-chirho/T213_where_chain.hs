-- TEST: compile_and_run
-- EXPECTED: 30
module Main where
f x = c where { c = a + b; a = x * 2; b = x * 4 }
main = print (f 5)
