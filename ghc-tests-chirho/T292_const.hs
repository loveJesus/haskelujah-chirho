-- TEST: compile_and_run
-- EXPECTED: 292
module Main where
const' x _ = x
main = print (const' 292 0)
