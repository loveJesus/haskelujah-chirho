-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
countIf :: (Int -> Bool) -> [Int] -> Int
countIf _ [] = 0
countIf p (x:xs) = if p x then 1 + countIf p xs else countIf p xs
main = print (countIf (\x -> x > 5) [3,7,2,9,1,8])
