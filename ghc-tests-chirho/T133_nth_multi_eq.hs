-- TEST: compile_and_run
-- EXPECTED: 10\n20\n30
module Main where
nth :: [Int] -> Int -> Int
nth (x:_) 0 = x
nth (_:xs) n = nth xs (n - 1)
nth [] _ = -1
main :: IO ()
main = do
  print (nth [10, 20, 30] 0)
  print (nth [10, 20, 30] 1)
  print (nth [10, 20, 30] 2)
