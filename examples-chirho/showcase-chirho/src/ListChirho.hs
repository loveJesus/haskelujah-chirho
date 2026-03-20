-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module ListChirho where

-- Quicksort
appendChirho :: [Int] -> [Int] -> [Int]
appendChirho [] ys = ys
appendChirho (x:xs) ys = x : appendChirho xs ys

qsortChirho :: [Int] -> [Int]
qsortChirho [] = []
qsortChirho (x:xs) = appendChirho (qsortChirho lo) (x : qsortChirho hi)
  where
    lo = myFilterChirho (\y -> y <= x) xs
    hi = myFilterChirho (\y -> y > x) xs

-- List operations
myMapChirho :: (Int -> Int) -> [Int] -> [Int]
myMapChirho _ [] = []
myMapChirho f (x:xs) = f x : myMapChirho f xs

myFilterChirho :: (Int -> Bool) -> [Int] -> [Int]
myFilterChirho _ [] = []
myFilterChirho p (x:xs) = if p x then x : myFilterChirho p xs else myFilterChirho p xs

myFoldlChirho :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldlChirho _ acc [] = acc
myFoldlChirho f acc (x:xs) = myFoldlChirho f (f acc x) xs

myLengthChirho :: [Int] -> Int
myLengthChirho [] = 0
myLengthChirho (_:xs) = 1 + myLengthChirho xs

enumFromToChirho :: Int -> Int -> [Int]
enumFromToChirho lo hi = if lo > hi then [] else lo : enumFromToChirho (lo + 1) hi
