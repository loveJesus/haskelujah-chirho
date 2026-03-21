-- TEST: compile_and_run
-- EXPECTED: 10
module Main where
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main = print (myLen [1,2,3,4,5,6,7,8,9,10])
