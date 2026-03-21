-- TEST: compile
-- Comprehensive smoke test: ADTs, guards, HOFs, where, IO, fib
module Main where
data Color = Red | Green | Blue deriving (Show, Eq)
data Shape = Circle Int | Rect Int Int
area :: Shape -> Int
area (Circle r) = r * r * 3; area (Rect w h) = w * h
classify n | n > 100 = "big" | n > 10 = "med" | otherwise = "small"
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myFoldl _ a [] = a; myFoldl f a (x:xs) = myFoldl f (f a x) xs
myLength [] = 0; myLength (_:xs) = 1 + myLength xs
isPrime n | n < 2 = False | otherwise = go 2
  where go d | d*d > n = True | n `mod` d == 0 = False | otherwise = go (d+1)
fib n = go n 0 1 where go 0 a _ = a; go n a b = go (n-1) b (a+b)
printItems :: [Int] -> IO ()
printItems [] = return (); printItems (x:xs) = do { print x; printItems xs }
main :: IO ()
main = do
  putStrLn (show Red)
  print (area (Circle 5))
  putStrLn (classify 50)
  putStrLn ("fib30: " ++ show (fib 30))
  printItems [1, 2, 3]
  putStrLn "ALL OK"
