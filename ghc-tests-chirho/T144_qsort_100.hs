-- TEST: compile
-- Quicksort on 100 LCG-generated elements
module Main where
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (x:xs) = append (qsort lo) (x : qsort hi)
  where lo = myFilter (\y -> y <= x) xs
        hi = myFilter (\y -> y > x) xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
lcg :: Int -> Int -> [Int]
lcg _ 0 = []
lcg seed n = (seed `mod` 1000) : lcg ((seed * 1103515245 + 12345) `mod` 2147483648) (n - 1)
main :: IO ()
main = print (mySum (qsort (lcg 42 100)))
