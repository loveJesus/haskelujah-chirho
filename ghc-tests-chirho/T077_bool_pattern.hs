-- TEST: compile_and_run
-- EXPECTED: 1\n0\n1\n0
module Main where
boolToInt :: Bool -> Int
boolToInt True = 1
boolToInt False = 0
main :: IO ()
main = do
  print (boolToInt (5 > 3))
  print (boolToInt (3 > 5))
  print (boolToInt (42 == 42))
  print (boolToInt (42 == 43))
