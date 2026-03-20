-- TEST: compile_and_run
-- EXPECTED: 1024\n6561\n100000
module Main where
power :: Int -> Int -> Int
power _ 0 = 1
power b e = b * power b (e - 1)
main :: IO ()
main = do
  print (power 2 10)
  print (power 3 8)
  print (power 10 5)
