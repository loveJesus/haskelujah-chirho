-- TEST: compile_and_run
-- EXPECTED: Hello\nWorld\nDone
module Main where
main :: IO ()
main = do
  putStrLn "Hello"
  putStrLn "World"
  putStrLn "Done"
