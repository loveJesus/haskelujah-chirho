-- TEST: compile_and_run
-- EXPECTED: Red\nGreen\nBlue
module Main where
data Color = Red | Green | Blue deriving (Show)
main :: IO ()
main = do
  print Red
  print Green
  print Blue
