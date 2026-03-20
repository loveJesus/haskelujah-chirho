-- TEST: compile_and_run
-- EXPECTED: 42
module Main where
add :: Int -> Int -> Int
add x y = x + y
add5 :: Int -> Int
add5 = add 5
main :: IO ()
main = print (add5 37)
