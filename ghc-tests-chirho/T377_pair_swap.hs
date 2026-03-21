-- TEST: compile_and_run
-- EXPECTED: 20\n10
module Main where
swap :: (Int, Int) -> (Int, Int)
swap (a, b) = (b, a)
main = do
  let (x, y) = swap (10, 20)
  print x
  print y
