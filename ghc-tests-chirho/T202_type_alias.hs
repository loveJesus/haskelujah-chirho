-- TEST: compile_and_run
-- EXPECTED: Hello World!
module Main where
type Name = String
greet :: Name -> String
greet n = "Hello " ++ n ++ "!"
main :: IO ()
main = putStrLn (greet "World")
