-- TEST: compile_and_run
-- EXPECTED: 9
module Main where
myFoldl _ acc [] = acc; myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
myMax a b = if a > b then a else b
main = print (myFoldl myMax 0 [3,7,2,9,1,5])
