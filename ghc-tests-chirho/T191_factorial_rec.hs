-- TEST: compile_and_run
-- EXPECTED: 3628800
module Main where
factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)
main = print (factorial 10)
