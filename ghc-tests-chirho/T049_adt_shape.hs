-- TEST: compile_and_run
-- EXPECTED: 147\n12
module Main where
data Shape = Circle Int | Rectangle Int Int
area :: Shape -> Int
area (Circle r) = r * r * 3
area (Rectangle w h) = w * h
main :: IO ()
main = do
  print (area (Circle 7))
  print (area (Rectangle 3 4))
