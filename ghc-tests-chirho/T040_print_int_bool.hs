-- TEST: compile_and_run
-- EXPECTED: 42\nTrue\n-7
module Main where
main :: IO ()
main = do
  print 42
  print True
  print (-7)
