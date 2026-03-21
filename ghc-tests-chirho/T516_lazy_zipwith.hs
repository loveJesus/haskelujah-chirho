-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [0,1,1,2,3,5,8,13,21,34]
module Main where
myZipWith :: (a -> b -> c) -> [a] -> [b] -> [c]
myZipWith _ [] _ = []
myZipWith _ _ [] = []
myZipWith f (x:xs) (y:ys) = f x y : myZipWith f xs ys
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
-- Fibonacci via lazy zipWith — classic Haskell idiom
fibs :: [Int]
fibs = 0 : 1 : myZipWith (+) fibs (myTail fibs)
myTail :: [a] -> [a]
myTail (_:xs) = xs
main = print (myTake 10 fibs)
