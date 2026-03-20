-- TEST: compile_and_run
-- EXPECTED: positive\nnegative\nzero
module Main where
main :: IO ()
main = do
  let classify n = case compare n 0 of
        GT -> "positive"
        LT -> "negative"
        EQ -> "zero"
  putStrLn (classify 5)
  putStrLn (classify (-3))
  putStrLn (classify 0)
  where
    compare a b
      | a > b = GT
      | a < b = LT
      | otherwise = EQ
