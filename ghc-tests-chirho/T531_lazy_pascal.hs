-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [1]\n[1,1]\n[1,2,1]\n[1,3,3,1]\n[1,4,6,4,1]\n[1,5,10,10,5,1]
module Main where
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
myZipWith :: (a -> b -> c) -> [a] -> [b] -> [c]
myZipWith _ [] _ = []
myZipWith _ _ [] = []
myZipWith f (x:xs) (y:ys) = f x y : myZipWith f xs ys
-- Infinite Pascal's triangle via lazy zipWith
nextRow :: [Int] -> [Int]
nextRow row = myZipWith (+) (0 : row) (row ++ [0])
pascal :: [[Int]]
pascal = [1] : myMap nextRow pascal
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
printRows :: [[Int]] -> IO ()
printRows [] = return ()
printRows (r:rs) = do
  print r
  printRows rs
main :: IO ()
main = printRows (myTake 6 pascal)
