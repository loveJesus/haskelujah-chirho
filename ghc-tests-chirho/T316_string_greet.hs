-- TEST: compile_and_run
-- EXPECTED: Hello, World!
module Main where
greet name = "Hello, " ++ name ++ "!"
main = putStrLn (greet "World")
