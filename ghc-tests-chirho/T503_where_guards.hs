-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: positive\nnegative\nzero
module Main where
classify x
  | x > 0     = "positive"
  | x < 0     = "negative"
  | otherwise  = "zero"
main = do
  putStrLn (classify 5)
  putStrLn (classify (-3))
  putStrLn (classify 0)
