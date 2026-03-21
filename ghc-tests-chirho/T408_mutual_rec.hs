-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
isEven_ 0 = True; isEven_ n = isOdd_ (n-1)
isOdd_ 0 = False; isOdd_ n = isEven_ (n-1)
main = do { print (bti (isEven_ 10)); print (bti (isEven_ 7)) }
