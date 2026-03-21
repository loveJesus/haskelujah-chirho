-- TEST: compile_and_run
-- EXPECTED: 143
module Main where
fibList n = go n 0 1 where go 0 _ _ = []; go n a b = a : go (n-1) b (a+b)
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (fibList 10))
