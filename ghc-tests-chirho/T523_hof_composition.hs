-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [4,16,36,64,100]\n55\n[10,9,8,7,6,5,4,3,2,1]
module Main where
compose :: (b -> c) -> (a -> b) -> a -> c
compose f g x = f (g x)
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
myFilter :: (a -> Bool) -> [a] -> [a]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myFoldr :: (a -> b -> b) -> b -> [a] -> b
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
square :: Int -> Int
square x = x * x
isEven :: Int -> Bool
isEven n = mod n 2 == 0
myReverse :: [a] -> [a]
myReverse xs = myFoldl consFlip [] xs
consFlip :: [a] -> a -> [a]
consFlip acc x = x : acc
myFoldl :: (b -> a -> b) -> b -> [a] -> b
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
main :: IO ()
main = do
  -- compose filter and map
  print (myMap square (myFilter isEven [1,2,3,4,5,6,7,8,9,10]))
  -- foldr for sum
  print (myFoldr (+) 0 [1,2,3,4,5,6,7,8,9,10])
  -- reverse via foldl
  print (myReverse [1,2,3,4,5,6,7,8,9,10])
