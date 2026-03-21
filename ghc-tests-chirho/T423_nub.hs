-- TEST: compile_and_run
-- EXPECTED: 5
module Main where
myElem _ [] = False; myElem x (y:ys) = if x == y then True else myElem x ys
nub [] = []; nub (x:xs) = if myElem x xs then nub xs else x : nub xs
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main = print (myLen (nub [1,2,3,2,1,4,3,5,1]))
