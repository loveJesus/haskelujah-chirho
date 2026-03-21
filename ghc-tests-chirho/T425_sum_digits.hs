-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
sumDigits :: Int -> Int
sumDigits n = if n < 10 then n else n `mod` 10 + sumDigits (n `div` 10)
main = print (sumDigits 12345)
