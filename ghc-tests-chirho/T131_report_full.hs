-- TEST: compile_and_run
-- EXPECTED: 7\n45\n12\n1
module Main where
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
myMax :: [Int] -> Int
myMax [x] = x
myMax (x:xs) = if x > myMax xs then x else myMax xs
myMax [] = 0
myMin :: [Int] -> Int
myMin [x] = x
myMin (x:xs) = if x < myMin xs then x else myMin xs
myMin [] = 0
main :: IO ()
main = do
  let xs = [7, 3, 12, 5, 9, 1, 8]
  print (myLength xs)
  print (mySum xs)
  print (myMax xs)
  print (myMin xs)
