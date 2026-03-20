-- TEST: compile_and_run
-- EXPECTED: FizzBuzz\nFizz\nBuzz\n7
module Main where
fizzbuzz :: Int -> String
fizzbuzz n
  | n `mod` 15 == 0 = "FizzBuzz"
  | n `mod` 3 == 0 = "Fizz"
  | n `mod` 5 == 0 = "Buzz"
  | otherwise = show n
main :: IO ()
main = do
  putStrLn (fizzbuzz 15)
  putStrLn (fizzbuzz 9)
  putStrLn (fizzbuzz 10)
  putStrLn (fizzbuzz 7)
