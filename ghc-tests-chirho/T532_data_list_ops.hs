-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 120\n[3,2,1]\n[1,2,3,4,5]
module Main where
myFoldl :: (b -> a -> b) -> b -> [a] -> b
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
myProduct :: [Int] -> Int
myProduct = myFoldl (*) 1
consFlip :: [a] -> a -> [a]
consFlip acc x = x : acc
myReverse :: [a] -> [a]
myReverse = myFoldl consFlip []
mySort :: [Int] -> [Int]
mySort [] = []
mySort (x:xs) = mySort smaller ++ [x] ++ mySort bigger
  where smaller = myFilter (< x) xs
        bigger = myFilter (>= x) xs
myFilter :: (a -> Bool) -> [a] -> [a]
myFilter _ [] = []
myFilter p (y:ys) = if p y then y : myFilter p ys else myFilter p ys
main :: IO ()
main = do
  print (myProduct [1,2,3,4,5])
  print (myReverse [1,2,3])
  print (mySort [3,1,4,5,2])
