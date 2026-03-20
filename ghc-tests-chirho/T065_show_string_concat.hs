-- TEST: compile_and_run
-- EXPECTED: result: 42 items
module Main where
main :: IO ()
main = putStrLn ("result: " ++ show 42 ++ " items")
