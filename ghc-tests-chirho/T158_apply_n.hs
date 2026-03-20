-- TEST: compile_and_run
-- EXPECTED: 1024\n32
module Main where
applyN :: Int -> (Int -> Int) -> Int -> Int
applyN 0 _ x = x
applyN n f x = applyN (n - 1) f (f x)
double :: Int -> Int
double x = x * 2
main :: IO ()
main = do
  print (applyN 10 double 1)
  print (applyN 5 double 1)
