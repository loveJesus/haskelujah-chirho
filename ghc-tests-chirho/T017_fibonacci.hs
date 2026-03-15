-- TEST: compile_and_run
-- EXPECT_OUTPUT: 55
-- Fibonacci
main = print (fib 10)

fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)
