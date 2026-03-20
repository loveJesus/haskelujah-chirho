-- TEST: compile_and_run
-- EXPECTED: Hello World!\nThe answer is 42
module Main where
greet :: String -> String
greet name = "Hello " ++ name ++ "!"
main :: IO ()
main = do
  putStrLn (greet "World")
  putStrLn ("The answer is " ++ show 42)
