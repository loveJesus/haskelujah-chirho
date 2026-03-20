-- TEST: compile_and_run
-- EXPECTED: 3\n2\n1\n0
module Main where
classify :: Int -> Int
classify x = if x > 100 then 3
             else if x > 10 then 2
             else if x > 0 then 1
             else 0
main :: IO ()
main = do
  print (classify 200)
  print (classify 50)
  print (classify 5)
  print (classify (-1))
