-- TEST: compile_and_run
-- EXPECTED: 5\n3
module Main where
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main = do
  print (myLen (myFilter (\x -> x > 0) [-2,3,-1,5,0,7,-4,8]))
  print (myLen (myFilter (\x -> x < 0) [-2,3,-1,5,0,7,-4,8]))
