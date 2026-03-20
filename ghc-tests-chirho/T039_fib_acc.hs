-- TEST: compile_and_run
-- EXPECTED: 12586269025
module Main where
fib :: Int -> Int
fib n = go n 0 1
  where go 0 a _ = a
        go n a b = go (n - 1) b (a + b)
main :: IO ()
main = print (fib 50)
