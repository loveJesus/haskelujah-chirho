-- TEST: compile_and_run
-- EXPECTED: 42\n7
module Main where
id' :: Int -> Int
id' x = x
const' :: Int -> Int -> Int
const' x _ = x
main :: IO ()
main = do
  print (id' 42)
  print (const' 7 99)
