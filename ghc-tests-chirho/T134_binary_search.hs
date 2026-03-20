-- TEST: compile_and_run
-- EXPECTED: 5\n8\n-1
module Main where
nth :: [Int] -> Int -> Int
nth (x:_) 0 = x
nth (_:xs) n = nth xs (n - 1)
nth [] _ = -1
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
bsearch :: [Int] -> Int -> Int -> Int -> Int
bsearch xs target lo hi
  | lo > hi = -1
  | nth xs mid == target = mid
  | nth xs mid < target = bsearch xs target (mid + 1) hi
  | otherwise = bsearch xs target lo (mid - 1)
  where mid = (lo + hi) `div` 2
main :: IO ()
main = do
  let sorted = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
  print (bsearch sorted 23 0 (myLength sorted - 1))
  print (bsearch sorted 72 0 (myLength sorted - 1))
  print (bsearch sorted 99 0 (myLength sorted - 1))
