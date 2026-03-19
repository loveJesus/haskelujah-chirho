-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

-- Fibonacci sequence
fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)

-- Collatz conjecture step counter
collatzSteps :: Int -> Int
collatzSteps n = go n 0
  where
    go 1 s = s
    go k s = if k `mod` 2 == 0
               then go (k `div` 2) (s + 1)
               else go (3 * k + 1) (s + 1)

-- Simple ADT
data Shape = Circle Int | Rectangle Int Int

area :: Shape -> Int
area (Circle r) = 3 * r * r
area (Rectangle w h) = w * h

-- Higher-order function
applyTwice :: (Int -> Int) -> Int -> Int
applyTwice f x = f (f x)

main :: IO ()
main = do
  putStrLn "=== Haskelujah Chirho Demo ==="
  putStrLn ""
  putStrLn "Fibonacci:"
  print (fib 10)
  print (fib 20)
  putStrLn ""
  putStrLn "Collatz steps:"
  print (collatzSteps 27)
  putStrLn ""
  putStrLn "Shapes:"
  print (area (Circle 5))
  print (area (Rectangle 3 7))
  putStrLn ""
  putStrLn "Higher-order functions:"
  print (applyTwice (\x -> x * 2) 3)
  putStrLn ""
  putStrLn "List operations:"
  print (sum [1, 2, 3, 4, 5])
  print (length [10, 20, 30, 40])
  putStrLn ""
  putStrLn "Done! Glory to God."
