-- TEST: compile_and_run
-- EXPECTED: 832040
module Main where
fib n = go n 0 1 where go 0 a _ = a; go n a b = go (n-1) b (a+b)
main = print (fib 30)
