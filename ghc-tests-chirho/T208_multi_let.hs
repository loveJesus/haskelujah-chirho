-- TEST: compile_and_run
-- EXPECTED: 30
module Main where
main :: IO ()
main = do
  let x = 10
  let y = 20
  print (x + y)
