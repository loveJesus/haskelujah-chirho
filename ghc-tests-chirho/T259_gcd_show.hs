-- TEST: compile_and_run
-- EXPECTED: gcd(48,18) = 6
module Main where
gcd' a 0 = a; gcd' a b = gcd' b (a `mod` b)
main = putStrLn ("gcd(48,18) = " ++ show (gcd' 48 18))
