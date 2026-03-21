-- TEST: compile_and_run
-- EXPECTED: result: 25
module Main where
compute :: Int -> String
compute x = "result: " ++ show result where result = a + b; a = x * 2; b = x * 3
main = putStrLn (compute 5)
