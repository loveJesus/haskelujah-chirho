-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [2,4,6,8,10]\n[2,4,6,8,10]\n[1,4,9,16,25]
module Main where
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
myFilter :: (a -> Bool) -> [a] -> [a]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
isEven :: Int -> Bool
isEven n = mod n 2 == 0
main = do
  print (myFilter isEven [1,2,3,4,5,6,7,8,9,10])
  print (myMap (* 2) [1,2,3,4,5])
  print (myMap (\x -> x * x) [1,2,3,4,5])
