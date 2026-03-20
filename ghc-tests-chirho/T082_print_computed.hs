-- TEST: compile_and_run
-- EXPECTED: 7\n50\n63
module Main where
main :: IO ()
main = do
  print (3 + 4)
  print (10 * 5)
  print (100 - 37)
