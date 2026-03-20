-- TEST: compile_and_run
-- EXPECTED: 21
module Main where
gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
main :: IO ()
main = print (gcd' 252 105)
