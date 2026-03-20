-- TEST: compile_and_run
-- EXPECTED: 12\n1
module Main where
myMax :: Int -> [Int] -> Int
myMax best [] = best
myMax best (x:xs) = if x > best then myMax x xs else myMax best xs
myMin :: Int -> [Int] -> Int
myMin best [] = best
myMin best (x:xs) = if x < best then myMin x xs else myMin best xs
main :: IO ()
main = do
  print (myMax 0 [7, 3, 12, 5, 9, 1, 8])
  print (myMin 999 [7, 3, 12, 5, 9, 1, 8])
