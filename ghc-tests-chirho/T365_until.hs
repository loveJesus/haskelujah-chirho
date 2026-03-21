-- TEST: compile_and_run
-- EXPECTED: 1024
module Main where
myUntil :: (Int -> Bool) -> (Int -> Int) -> Int -> Int
myUntil p f x = if p x then x else myUntil p f (f x)
main = print (myUntil (\x -> x >= 1000) (\x -> x * 2) 1)
