-- TEST: compile_and_run
-- EXPECTED: 6\n22
module Main where
myTakeWhile _ [] = []; myTakeWhile p (x:xs) = if p x then x : myTakeWhile p xs else []
myDropWhile _ [] = []; myDropWhile p (x:xs) = if p x then myDropWhile p xs else x : xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = do
  let xs = [1,2,3,4,5,6,7]
  print (mySum (myTakeWhile (<=3) xs))
  print (mySum (myDropWhile (<=3) xs))
