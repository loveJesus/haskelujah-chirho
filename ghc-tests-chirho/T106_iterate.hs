-- TEST: compile_and_run
-- EXPECTED: 1024
module Main where
iterateN :: Int -> (Int -> Int) -> Int -> Int
iterateN 0 _ x = x
iterateN n f x = iterateN (n - 1) f (f x)
double :: Int -> Int
double x = x * 2
main :: IO ()
main = print (iterateN 10 double 1)
