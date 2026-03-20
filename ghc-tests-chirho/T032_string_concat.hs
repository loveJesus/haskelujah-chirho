-- TEST: compile_and_run
-- EXPECTED: Hello World
module Main where
main :: IO ()
main = putStrLn ("Hello" ++ " " ++ "World")
