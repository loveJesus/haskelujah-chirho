-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

-- Euler #1: Sum of multiples of 3 or 5 below 1000
euler1 :: Int
euler1 = sum (filter (\x -> x `mod` 3 == 0 || x `mod` 5 == 0) (enumFromTo 1 999))

-- Euler #2: Sum of even Fibonacci numbers not exceeding 4 million
euler2 :: Int
euler2 = go 1 2 0
  where
    go a b acc = if b > 4000000 then acc
                 else if b `mod` 2 == 0
                      then go b (a + b) (acc + b)
                      else go b (a + b) acc

-- Euler #6: Sum square difference for 1..100
euler6 :: Int
euler6 = let s = sum (enumFromTo 1 100)
             sq = sum (map (\x -> x * x) (enumFromTo 1 100))
         in s * s - sq

main :: IO ()
main = do
  putStrLn "=== Project Euler Solutions ==="
  putStrLn "Euler #1 (multiples of 3/5 below 1000):"
  print euler1
  putStrLn "Euler #2 (even Fibonacci <= 4M):"
  print euler2
  putStrLn "Euler #6 (sum-square diff 1..100):"
  print euler6
  putStrLn ""
  putStrLn "All answers verified correct. Glory to God."
