-- TEST: compile_and_run
-- EXPECTED: 35
module Main where
runningSum :: [Int] -> [Int]
runningSum xs = go xs 0
  where
    go [] _ = []
    go (x:rest) acc = let acc' = acc + x in acc' : go rest acc'
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (runningSum [1, 2, 3, 4, 5]))
