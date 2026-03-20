-- TEST: compile_and_run
-- EXPECTED: 45
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
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (qsort [5, 3, 8, 1, 9, 2, 7, 4, 6]))
