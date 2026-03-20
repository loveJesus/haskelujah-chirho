-- TEST: compile_and_run
-- EXPECTED: 9
module Main where
myMax :: [Int] -> Int
myMax [x] = x
myMax (x:xs) = if x > myMax xs then x else myMax xs
myMax [] = 0
main :: IO ()
main = print (myMax [5, 3, 9, 2])
