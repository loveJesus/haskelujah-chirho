-- TEST: compile_and_run
-- EXPECTED: 6\n12
module Main where
myTake :: Int -> [Int] -> [Int]
myTake 0 _ = []
myTake _ [] = []
myTake n (x:xs) = x : myTake (n - 1) xs
myDrop :: Int -> [Int] -> [Int]
myDrop 0 xs = xs
myDrop _ [] = []
myDrop n (_:xs) = myDrop (n - 1) xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = do
  print (mySum (myTake 3 [1, 2, 3, 4, 5]))
  print (mySum (myDrop 3 [1, 2, 3, 4, 5]))
