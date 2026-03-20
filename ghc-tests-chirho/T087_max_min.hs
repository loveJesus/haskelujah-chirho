-- TEST: compile_and_run
-- EXPECTED: 10\n1
module Main where
myMax :: Int -> Int -> Int
myMax a b = if a >= b then a else b
myMin :: Int -> Int -> Int
myMin a b = if a <= b then a else b
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
main :: IO ()
main = do
  print (myFoldl myMax 0 [3, 7, 2, 10, 5])
  print (myFoldl myMin 100 [8, 3, 1, 9, 4])
