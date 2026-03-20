-- TEST: compile_and_run
-- EXPECTED: 100\n0\n42
module Main where
f :: Int -> Int
f (-1) = 100
f 0 = 0
f n = n
main :: IO ()
main = do
  print (f (-1))
  print (f 0)
  print (f 42)
