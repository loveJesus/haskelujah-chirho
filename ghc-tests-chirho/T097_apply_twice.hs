-- TEST: compile_and_run
-- EXPECTED: 18
module Main where
twice :: (Int -> Int) -> Int -> Int
twice f x = f (f x)
triple :: Int -> Int
triple x = x * 3
main :: IO ()
main = print (twice triple 2)
