-- TEST: compile_and_run
-- EXPECTED: 120
module Main where
myFoldr _ z [] = z; myFoldr f z (x:xs) = f x (myFoldr f z xs)
mul a b = a * b
main = print (myFoldr mul 1 [1,2,3,4,5])
