-- TEST: compile_and_run
-- EXPECTED: 1024
module Main where
power :: Int -> Int -> Int
power _ 0 = 1
power b e = b * power b (e - 1)
main :: IO ()
main = print (power 2 10)
