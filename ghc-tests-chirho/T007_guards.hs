-- TEST: compile_and_run
-- EXPECT_OUTPUT: fizzbuzz
-- Guards
main = putStrLn (fizzbuzz 15)

fizzbuzz :: Int -> String
fizzbuzz n
  | n `mod` 15 == 0 = "fizzbuzz"
  | n `mod` 3  == 0 = "fizz"
  | n `mod` 5  == 0 = "buzz"
  | otherwise       = show n
