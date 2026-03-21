-- TEST: compile_and_run
-- EXPECTED: 6
module Main where
gcd' a 0 = a; gcd' a b = gcd' b (a `mod` b)
main = print (gcd' 48 18)
