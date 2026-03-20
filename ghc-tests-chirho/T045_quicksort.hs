-- TEST: compile
-- Quicksort with filter, append, lambda HOFs should compile
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
  where
    lo = myFilter (\y -> y <= x) xs
    hi = myFilter (\y -> y > x) xs
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
main :: IO ()
main = print (sumList (qsort [5, 3, 8, 1, 9, 2, 7, 4, 6]))
