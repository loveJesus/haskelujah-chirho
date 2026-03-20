-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
myAny _ [] = False; myAny p (x:xs) = if p x then True else myAny p xs
main = do { print (bti (myAny (\x -> x > 5) [1,2,3,7])); print (bti (myAny (\x -> x > 10) [1,2,3])) }
