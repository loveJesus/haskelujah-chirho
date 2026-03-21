-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [1,2,3,4,5,6,8,9,10,12,15,16,18,20,24,25,27,30,32,36]
module Main where
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
myMerge :: [Int] -> [Int] -> [Int]
myMerge (x:xs) (y:ys)
  | x < y = x : myMerge xs (y:ys)
  | x > y = y : myMerge (x:xs) ys
  | otherwise = x : myMerge xs ys
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
-- Hamming numbers: numbers whose only prime factors are 2, 3, 5
-- Classic lazy evaluation example
hamming :: [Int]
hamming = 1 : myMerge (myMap (* 2) hamming)
                       (myMerge (myMap (* 3) hamming) (myMap (* 5) hamming))
main = print (myTake 20 hamming)
