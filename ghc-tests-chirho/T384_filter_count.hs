-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
main = print (myLen (myFilter (>3) [1,2,3,4,5,6]))
