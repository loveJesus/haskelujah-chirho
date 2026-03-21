-- TEST: compile_and_run
-- EXPECTED: Red\n1\n0
module Main where
data Color = Red | Green | Blue deriving (Show, Eq)
bti True = 1; bti False = 0
main = do
  putStrLn (show Red)
  print (bti (Red == Red))
  print (bti (Red == Blue))
