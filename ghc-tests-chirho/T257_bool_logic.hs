-- TEST: compile_and_run
-- EXPECTED: 1\n0\n0\n1
module Main where
bti True = 1; bti False = 0
myAnd True True = True; myAnd _ _ = False
myOr False False = False; myOr _ _ = True
main = do
  print (bti (myAnd True True))
  print (bti (myAnd True False))
  print (bti (myOr False False))
  print (bti (myOr True False))
