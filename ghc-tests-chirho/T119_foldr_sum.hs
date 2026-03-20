-- TEST: compile_and_run
-- EXPECTED: 15\n720
module Main where
myFoldr :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
add :: Int -> Int -> Int
add x y = x + y
mul :: Int -> Int -> Int
mul x y = x * y
main :: IO ()
main = do
  print (myFoldr add 0 [1, 2, 3, 4, 5])
  print (myFoldr mul 1 [1, 2, 3, 4, 5, 6])
