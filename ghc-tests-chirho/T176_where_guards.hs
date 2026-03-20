-- TEST: compile_and_run
-- EXPECTED: 5\n0\n0
module Main where
f :: Int -> Int
f x
  | a > 0 = a
  | otherwise = 0
  where a = x - 5
main :: IO ()
main = do
  print (f 10)
  print (f 3)
  print (f 5)
