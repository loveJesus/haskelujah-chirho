-- TEST: compile_and_run
-- EXPECTED: LT\nEQ\nGT
module Main where
main :: IO ()
main = do
  putStrLn (show LT)
  putStrLn (show EQ)
  putStrLn (show GT)
