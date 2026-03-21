-- TEST: compile_and_run
-- EXPECTED: f(3) = 9, f(4) = 16
module Main where
f x = x * x
main = putStrLn ("f(3) = " ++ show (f 3) ++ ", f(4) = " ++ show (f 4))
