-- TEST: compile_and_run
-- EXPECTED: 3 r 2
module Main where
divmod :: Int -> Int -> String
divmod a b = show q ++ " r " ++ show r where q = a `div` b; r = a `mod` b
main = putStrLn (divmod 17 5)
