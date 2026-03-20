-- TEST: compile_and_run
-- EXPECTED: 147\n0\n12\n0
module Main where
data Shape = Circle Int | Rectangle Int Int
area :: Shape -> Int
area (Circle r)
  | r > 0 = r * r * 3
  | otherwise = 0
area (Rectangle w h)
  | w > 0 = w * h
  | otherwise = 0
main :: IO ()
main = do
  print (area (Circle 7))
  print (area (Circle (-1)))
  print (area (Rectangle 3 4))
  print (area (Rectangle (-1) 5))
