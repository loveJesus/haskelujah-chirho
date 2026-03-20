-- TEST: compile_and_run
-- EXPECTED: Hello World!
module Main where
greet :: String -> String
greet name = msg
  where msg = "Hello " ++ name ++ "!"
main :: IO ()
main = putStrLn (greet "World")
