-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
myReverse xs = go xs [] where go [] a = a; go (x:rest) a = go rest (x:a)
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (myReverse [1,2,3,4,5]))
