-- TEST: compile_and_run
-- EXPECTED: 35
module Main where
iterCollect :: Int -> (Int -> Int) -> Int -> [Int]
iterCollect 0 _ _ = []
iterCollect n f x = x : iterCollect (n-1) f (f x)
mySum [] = 0; mySum (x:xs) = x + mySum xs
main = print (mySum (iterCollect 5 (+1) 5))
