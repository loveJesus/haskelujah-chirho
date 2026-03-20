-- TEST: compile_and_run
-- EXPECTED: -42\n42
module Main where
negate' x = 0 - x
main = do { print (negate' 42); print (negate' (-42)) }
