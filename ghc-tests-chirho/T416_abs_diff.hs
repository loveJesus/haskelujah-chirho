-- TEST: compile_and_run
-- EXPECTED: 5\n5\n0
module Main where
absDiff a b = if a > b then a - b else b - a
main = do { print (absDiff 10 5); print (absDiff 3 8); print (absDiff 7 7) }
