-- TEST: compile_and_run
-- EXPECTED: !!!!!
module Main where
repeatStr 0 _ = ""; repeatStr n s = s ++ repeatStr (n-1) s
main = putStrLn (repeatStr 5 "!")
