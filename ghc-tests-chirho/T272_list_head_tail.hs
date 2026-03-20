-- TEST: compile_and_run
-- EXPECTED: 1\n4
module Main where
myHead (x:_) = x; myHead [] = 0
myTail (_:xs) = xs; myTail [] = []
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main = do { print (myHead [1,2,3]); print (myLen (myTail [1,2,3,4,5])) }
