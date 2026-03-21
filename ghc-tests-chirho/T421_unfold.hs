-- TEST: compile_and_run
-- EXPECTED: 55
module Main where
unfold :: (Int -> Bool) -> (Int -> Int) -> (Int -> Int) -> Int -> [Int]
unfold done val next seed = if done seed then [] else val seed : unfold done val next (next seed)
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (unfold (>10) (\x -> x) (+1) 1))
