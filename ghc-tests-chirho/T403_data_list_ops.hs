-- TEST: compile_and_run
-- EXPECTED: 3\n1\n5
module Main where
myHead (x:_) = x; myHead [] = 0
myLast [x] = x; myLast (_:xs) = myLast xs; myLast [] = 0
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main = do { print (myHead [3,4,5]); print (myLast [3,4,1]); print (myLen [1,2,3,4,5]) }
