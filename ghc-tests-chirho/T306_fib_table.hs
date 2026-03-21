-- TEST: compile_and_run
-- EXPECTED: fib(1) = 1\nfib(2) = 1\nfib(3) = 2\nfib(4) = 3\nfib(5) = 5
module Main where
fibTable :: Int -> IO ()
fibTable n = go 1
  where
    fib 0 = 0
    fib 1 = 1
    fib k = fib (k-1) + fib (k-2)
    go i
      | i > n = return ()
      | otherwise = do
          putStrLn ("fib(" ++ show i ++ ") = " ++ show (fib i))
          go (i + 1)
main :: IO ()
main = fibTable 5
