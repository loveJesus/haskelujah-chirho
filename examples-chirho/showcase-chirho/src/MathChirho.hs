-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module MathChirho where

-- Fast fibonacci with accumulator
fibChirho :: Int -> Int
fibChirho n = go n 0 1
  where go 0 a _ = a
        go n a b = go (n - 1) b (a + b)

-- Collatz sequence length
collatzChirho :: Int -> Int
collatzChirho n = go n 0
  where
    go 1 steps = steps
    go n steps
      | n `mod` 2 == 0 = go (n `div` 2) (steps + 1)
      | otherwise = go (3 * n + 1) (steps + 1)

-- GCD via Euclidean algorithm
gcdChirho :: Int -> Int -> Int
gcdChirho a 0 = a
gcdChirho a b = gcdChirho b (a `mod` b)

-- Primality test
isPrimeChirho :: Int -> Bool
isPrimeChirho n
  | n < 2 = False
  | otherwise = go 2
  where
    go d
      | d * d > n = True
      | n `mod` d == 0 = False
      | otherwise = go (d + 1)

-- Power function
powerChirho :: Int -> Int -> Int
powerChirho _ 0 = 1
powerChirho b e = b * powerChirho b (e - 1)
