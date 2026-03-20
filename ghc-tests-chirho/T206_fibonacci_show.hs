-- TEST: compile_and_run
-- EXPECTED: fib(30) = 832040
module Main where
fib :: Int -> Int
fib n = go n 0 1
  where go 0 a _ = a
        go n a b = go (n-1) b (a+b)
main = putStrLn ("fib(30) = " ++ show (fib 30))
