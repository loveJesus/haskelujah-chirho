-- TEST: compile_and_run
-- EXPECTED: 7\n45\n12\n1
module Main where
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
myMax :: Int -> [Int] -> Int
myMax best [] = best
myMax best (x:xs) = if x > best then myMax x xs else myMax best xs
myMin :: Int -> [Int] -> Int
myMin best [] = best
myMin best (x:xs) = if x < best then myMin x xs else myMin best xs
main :: IO ()
main = do
  let xs = [7, 3, 12, 5, 9, 1, 8]
  print (myLength xs)
  print (mySum xs)
  print (myMax 0 xs)
  print (myMin 999 xs)
