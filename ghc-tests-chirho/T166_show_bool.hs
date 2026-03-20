-- TEST: compile_and_run
-- EXPECTED: True\nFalse
module Main where
main :: IO ()
main = do
  putStrLn (show True)
  putStrLn (show False)
