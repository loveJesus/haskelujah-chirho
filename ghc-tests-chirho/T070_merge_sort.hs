-- TEST: compile
-- Merge sort with take/drop/merge/length/where
module Main where
merge :: [Int] -> [Int] -> [Int]
merge [] ys = ys
merge xs [] = xs
merge (x:xs) (y:ys) = if x <= y then x : merge xs (y:ys) else y : merge (x:xs) ys
myTake :: Int -> [Int] -> [Int]
myTake 0 _ = []
myTake _ [] = []
myTake n (x:xs) = x : myTake (n - 1) xs
myDrop :: Int -> [Int] -> [Int]
myDrop 0 xs = xs
myDrop _ [] = []
myDrop n (_:xs) = myDrop (n - 1) xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
msort :: [Int] -> [Int]
msort [] = []
msort (x:[]) = [x]
msort xs = merge (msort (myTake half xs)) (msort (myDrop half xs))
  where half = myLength xs `div` 2
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (msort [5, 3, 8, 1, 4, 2, 7, 6]))
