-- TEST: compile_and_run
-- EXPECTED: fib(1) = 1\nfib(2) = 1\nfib(3) = 2
module Main where
fib 0 = 0; fib 1 = 1; fib n = fib (n-1) + fib (n-2)
printFibs :: Int -> Int -> IO ()
printFibs n limit | n > limit = return () | otherwise = do
  putStrLn ("fib(" ++ show n ++ ") = " ++ show (fib n))
  printFibs (n+1) limit
main = printFibs 1 3
