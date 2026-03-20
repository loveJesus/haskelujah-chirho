-- TEST: compile_and_run
-- EXPECTED: 12586269025\n111\n21\n25\n45
-- Ultimate comprehensive test
module Main where
fib :: Int -> Int
fib n = go n 0 1
  where go 0 a _ = a
        go n a b = go (n-1) b (a+b)
collatz :: Int -> Int
collatz n = go n 0
  where go 1 s = s
        go n s
          | n `mod` 2 == 0 = go (n `div` 2) (s+1)
          | otherwise = go (3*n+1) (s+1)
gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
isPrime :: Int -> Bool
isPrime n
  | n < 2 = False
  | otherwise = go 2
  where go d | d*d > n = True
             | n `mod` d == 0 = False
             | otherwise = go (d+1)
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (x:xs) = append (qsort lo) (x : qsort hi)
  where lo = myFilter (\y -> y <= x) xs
        hi = myFilter (\y -> y > x) xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
main :: IO ()
main = do
  print (fib 50)
  print (collatz 27)
  print (gcd' 252 105)
  print (myLength (myFilter isPrime (enumFromTo 2 100)))
  print (mySum (qsort [5,3,8,1,9,2,7,4,6]))
