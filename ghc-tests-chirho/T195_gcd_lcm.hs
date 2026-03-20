-- TEST: compile_and_run
-- EXPECTED: 6\n60
module Main where
gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
lcm' :: Int -> Int -> Int
lcm' a b = (a * b) `div` gcd' a b
main = do { print (gcd' 12 18); print (lcm' 12 20) }
