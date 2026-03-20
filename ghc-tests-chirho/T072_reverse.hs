-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
myReverse :: [Int] -> [Int]
myReverse xs = go xs []
  where go [] acc = acc
        go (x:rest) acc = go rest (x : acc)
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = print (mySum (myReverse [1, 2, 3, 4, 5]))
