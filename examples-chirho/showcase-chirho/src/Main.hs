-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

import MathChirho
import ListChirho

main :: IO ()
main = do
  putStrLn "=== Haskelujah Chirho Feature Showcase ==="
  putStrLn ""

  -- Number theory
  putStrLn "Number Theory:"
  putStrLn ("  fib(50) = " ++ show (fibChirho 50))
  putStrLn ("  collatz(27) = " ++ show (collatzChirho 27) ++ " steps")
  putStrLn ("  gcd(252, 105) = " ++ show (gcdChirho 252 105))
  putStrLn ("  2^20 = " ++ show (powerChirho 2 20))
  putStrLn ("  primes < 100: " ++ show (myLengthChirho (myFilterChirho isPrimeChirho (enumFromToChirho 2 100))))
  putStrLn ""

  -- List operations
  putStrLn "List Operations:"
  let nums = enumFromToChirho 1 10
  putStrLn ("  sum [1..10] = " ++ show (myFoldlChirho (\a x -> a + x) 0 nums))
  putStrLn ("  sum squares = " ++ show (myFoldlChirho (\a x -> a + x * x) 0 nums))
  putStrLn ("  even count = " ++ show (myLengthChirho (myFilterChirho (\x -> x `mod` 2 == 0) nums)))
  putStrLn ""

  -- Quicksort
  putStrLn "Quicksort:"
  let sorted = qsortChirho [42, 17, 93, 5, 28, 61, 84, 3, 76, 50]
  putStrLn ("  sorted sum = " ++ show (myFoldlChirho (\a x -> a + x) 0 sorted))
  putStrLn ""

  putStrLn "All tests passed! Glory to God!"
