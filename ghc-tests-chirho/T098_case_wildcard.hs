-- TEST: compile_and_run
-- EXPECTED: fizz\none\nother
module Main where
classify :: Int -> String
classify n = case n `mod` 3 of
  0 -> "fizz"
  1 -> "one"
  _ -> "other"
main :: IO ()
main = do
  putStrLn (classify 9)
  putStrLn (classify 7)
  putStrLn (classify 8)
