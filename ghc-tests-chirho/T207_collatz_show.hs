-- TEST: compile_and_run
-- EXPECTED: collatz(27) = 111 steps
module Main where
collatz :: Int -> Int
collatz n = go n 0
  where go 1 s = s
        go n s | n `mod` 2 == 0 = go (n `div` 2) (s+1) | otherwise = go (3*n+1) (s+1)
main = putStrLn ("collatz(27) = " ++ show (collatz 27) ++ " steps")
