-- TEST: compile
-- Polymorphic map used at Int->Int and Int->Bool
module Main where
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
double :: Int -> Int
double x = x * 2
isEven :: Int -> Bool
isEven n = n `mod` 2 == 0
main :: IO ()
main = do
  print (myMap double [1, 2, 3])
  print (myMap isEven [1, 2, 3])
