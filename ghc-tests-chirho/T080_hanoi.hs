-- TEST: compile_and_run
-- EXPECTED: 1023
module Main where
hanoi :: Int -> Int
hanoi 0 = 0
hanoi n = 2 * hanoi (n - 1) + 1
main :: IO ()
main = print (hanoi 10)
