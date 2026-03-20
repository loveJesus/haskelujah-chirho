-- TEST: compile_and_run
-- EXPECTED: The answer is: 42
module Main where
main :: IO ()
main = putStrLn ("The answer is: " ++ show 42)
