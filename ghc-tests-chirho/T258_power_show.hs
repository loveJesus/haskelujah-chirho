-- TEST: compile_and_run
-- EXPECTED: 2^10 = 1024
module Main where
power _ 0 = 1; power b e = b * power b (e-1)
main = putStrLn ("2^10 = " ++ show (power 2 10))
