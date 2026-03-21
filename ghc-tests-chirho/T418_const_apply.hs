-- TEST: compile_and_run
-- EXPECTED: 42\n42
module Main where
const_ x _ = x
main = do { print (const_ 42 99); print (const_ 42 0) }
