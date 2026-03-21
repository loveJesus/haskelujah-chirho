-- TEST: compile_and_run
-- EXPECTED: Red\n75\nmed\nprimes: 10\nfib30: 832040\nALL OK
module Main where
data Color = Red | Green | Blue deriving (Show, Eq)
data Shape = Circle Int | Rect Int Int
area (Circle r) = r * r * 3; area (Rect w h) = w * h
classify n | n > 100 = "big" | n > 10 = "med" | otherwise = "small"
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myLength [] = 0; myLength (_:xs) = 1 + myLength xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
isPrime n | n < 2 = False | otherwise = go 2
  where go d | d*d > n = True | n `mod` d == 0 = False | otherwise = go (d+1)
fib n = go n 0 1 where go 0 a _ = a; go n a b = go (n-1) b (a+b)
main :: IO ()
main = do
  putStrLn (show Red)
  print (area (Circle 5))
  putStrLn (classify 50)
  putStrLn ("primes: " ++ show (myLength (myFilter isPrime (enumFromTo 2 30))))
  putStrLn ("fib30: " ++ show (fib 30))
  putStrLn "ALL OK"
