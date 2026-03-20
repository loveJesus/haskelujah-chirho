-- TEST: compile_and_run
-- EXPECTED: 7\n0\n3
module Main where
myAbs :: Int -> Int
myAbs n | n >= 0 = n | otherwise = 0 - n
main = do { print (myAbs (-7)); print (myAbs 0); print (myAbs 3) }
