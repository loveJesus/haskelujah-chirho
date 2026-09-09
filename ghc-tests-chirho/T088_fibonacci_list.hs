-- TEST: compile_and_run
-- EXPECTED: 88
module Main where
fibList :: Int -> [Int]
fibList n = go n 0 1
  where
    go 0 _ _ = []
    go n a b = a : go (n - 1) b (a + b)
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (fibList 10))
