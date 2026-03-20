-- TEST: compile_and_run
-- EXPECTED: 10\n16
module Main where
main :: IO ()
main = do
  let a = let b = 5 in b * 2
  print a
  let x = let y = let z = 3 in z + 1 in y * y
  print x
