-- TEST: compile_and_run
-- EXPECTED: 5\n4\n3\n2\n1\ngo!
module Main where
countdown 0 = putStrLn "go!"; countdown n = do { print n; countdown (n-1) }
main = countdown 5
