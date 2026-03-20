-- TEST: compile_and_run
-- EXPECTED: 0\n5\n3
module Main where
myLen :: [Int] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = do { print (myLen []); print (myLen [1,2,3,4,5]); print (myLen [10,20,30]) }
