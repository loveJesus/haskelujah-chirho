-- TEST: compile_and_run
-- EXPECTED: 25
module Main where
countPrimes :: Int -> Int
countPrimes limit = sieve 2 0
  where
    sieve n count
      | n >= limit = count
      | isPrime n = sieve (n + 1) (count + 1)
      | otherwise = sieve (n + 1) count
    isPrime n = checkDiv n 2
    checkDiv n d
      | d * d > n = True
      | n `mod` d == 0 = False
      | otherwise = checkDiv n (d + 1)
main :: IO ()
main = print (countPrimes 100)
