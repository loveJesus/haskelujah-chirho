-- TEST: compile_and_run
-- EXPECTED: 42
module Main where
apply :: (Int -> Int) -> Int -> Int
apply f x = f x
double :: Int -> Int
double x = x * 2
main :: IO ()
main = print (apply double 21)
