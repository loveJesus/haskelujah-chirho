-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
append [] ys = ys; append (x:xs) ys = x : append xs ys
main = print (myLen (append [1,2] [3]))
