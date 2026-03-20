-- TEST: compile_and_run
-- EXPECTED: 1\n0\n-1
module Main where
sign :: Int -> Int
sign n
  | n > 0 = 1
  | n == 0 = 0
  | otherwise = -1
main :: IO ()
main = do
  print (sign 5)
  print (sign 0)
  print (sign (-3))
