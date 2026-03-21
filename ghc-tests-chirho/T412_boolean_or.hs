-- TEST: compile_and_run
-- EXPECTED: 1\n1\n1\n0
module Main where
bti True = 1; bti False = 0
myOr False False = False; myOr _ _ = True
main = do { print (bti (myOr True True)); print (bti (myOr True False)); print (bti (myOr False True)); print (bti (myOr False False)) }
