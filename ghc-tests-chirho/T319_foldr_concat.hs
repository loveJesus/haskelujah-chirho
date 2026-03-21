-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
myFoldr _ z [] = z; myFoldr f z (x:xs) = f x (myFoldr f z xs)
add a b = a + b
main = print (myFoldr add 0 [1,2,3,4,5])
