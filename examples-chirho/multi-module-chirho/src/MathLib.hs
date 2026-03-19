-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module MathLib (factorial, isPrime, gcd') where

factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)

isPrime :: Int -> Int
isPrime n = if go n 2 then 1 else 0
  where
    go k d = if d * d > k then 1 == 1
             else if k `mod` d == 0 then 1 == 0
             else go k (d + 1)

gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
