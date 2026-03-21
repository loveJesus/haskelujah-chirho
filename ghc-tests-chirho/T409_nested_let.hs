-- TEST: compile_and_run
-- EXPECTED: 16
module Main where
main = do
  let x = let y = let z = 3 in z + 1 in y * y
  print x
