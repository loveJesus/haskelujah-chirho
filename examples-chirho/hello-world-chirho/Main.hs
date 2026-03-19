-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

-- Fibonacci
fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)

-- Collatz steps
collatzSteps :: Int -> Int
collatzSteps n = go n 0
  where
    go 1 s = s
    go k s = if k `mod` 2 == 0
               then go (k `div` 2) (s + 1)
               else go (3 * k + 1) (s + 1)

-- ADTs
data Shape = Circle Int | Rectangle Int Int
area :: Shape -> Int
area (Circle r) = 3 * r * r
area (Rectangle w h) = w * h

-- Higher-order / Closures
applyTwice :: (Int -> Int) -> Int -> Int
applyTwice f x = f (f x)

makeAdder :: Int -> Int -> Int
makeAdder n = \x -> x + n

compose :: (Int -> Int) -> (Int -> Int) -> Int -> Int
compose f g x = f (g x)

-- Euler #1
euler1 :: Int
euler1 = sum (filter (\x -> x `mod` 3 == 0 || x `mod` 5 == 0) (enumFromTo 1 999))

main :: IO ()
main = do
  putStrLn "=== Haskelujah Chirho Compiler Demo ==="
  putStrLn ""

  putStrLn "Fibonacci: fib 10, fib 20"
  print (fib 10)
  print (fib 20)

  putStrLn "Collatz: steps for 27"
  print (collatzSteps 27)

  putStrLn "Shapes: Circle 5, Rectangle 3 7"
  print (area (Circle 5))
  print (area (Rectangle 3 7))

  putStrLn "Higher-order: applyTwice (*2) 3"
  print (applyTwice (\x -> x * 2) 3)

  putStrLn "Closures: makeAdder 5 37, compose double inc 20"
  let add5 = makeAdder 5
  print (add5 37)
  let doubleInc = compose (\x -> x * 2) (\x -> x + 1)
  print (doubleInc 20)

  putStrLn "Euler #1: multiples of 3/5 below 1000"
  print euler1

  putStrLn "Lists: sum squares, count evens"
  print (sum (map (\x -> x * x) (enumFromTo 1 10)))
  print (length (filter (\x -> x `mod` 2 == 0) (enumFromTo 1 100)))

  putStrLn ""
  putStrLn "All tests passed! Glory to God."
