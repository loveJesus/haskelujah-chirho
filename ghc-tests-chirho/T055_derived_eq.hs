-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
data Color = Red | Green | Blue deriving (Eq)
boolToInt :: Bool -> Int
boolToInt True = 1
boolToInt False = 0
main :: IO ()
main = do
  print (boolToInt (Red == Red))
  print (boolToInt (Red == Blue))
