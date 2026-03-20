-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
myAll :: (Int -> Bool) -> [Int] -> Bool
myAll _ [] = True
myAll p (x:xs) = if p x then myAll p xs else False
isPositive :: Int -> Bool
isPositive n = n > 0
boolToInt :: Bool -> Int
boolToInt True = 1
boolToInt False = 0
main :: IO ()
main = do
  print (boolToInt (myAll isPositive [1, 2, 3]))
  print (boolToInt (myAll isPositive [1, -2, 3]))
