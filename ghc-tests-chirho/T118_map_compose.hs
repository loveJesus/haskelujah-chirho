-- TEST: compile_and_run
-- EXPECTED: 33
module Main where
compose :: (Int -> Int) -> (Int -> Int) -> Int -> Int
compose f g x = f (g x)
inc :: Int -> Int
inc x = x + 1
double :: Int -> Int
double x = x * 2
square :: Int -> Int
square x = x * x
main :: IO ()
main = print (compose inc (compose double square) 4)
