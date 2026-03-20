-- TEST: compile_and_run
-- EXPECTED: 12\n36
module Main where
compose :: (Int -> Int) -> (Int -> Int) -> Int -> Int
compose f g x = f (g x)
double :: Int -> Int
double x = x * 2
inc :: Int -> Int
inc x = x + 1
square :: Int -> Int
square x = x * x
main :: IO ()
main = do
  print (compose double inc 5)
  print (compose square double 3)
