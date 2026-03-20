-- TEST: compile_and_run
-- EXPECTED: 33
module Main where
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
myFilter _ [] = []; myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo+1) hi
isFizz n = n `mod` 3 == 0
main = print (myLen (myFilter isFizz (enumFromTo 1 100)))
