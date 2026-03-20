-- TEST: compile_and_run
-- EXPECTED: 7\n7
module Main where
curry' f x y = f (x, y)
uncurry' f (x, y) = f x y
add x y = x + y
addPair (x, y) = x + y
main = do
  print (curry' addPair 3 4)
  print (uncurry' add (3, 4))
