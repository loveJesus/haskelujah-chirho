-- TEST: compile_and_run
-- EXPECTED: 2
module Main where
flip' :: (Int -> Int -> Int) -> Int -> Int -> Int
flip' f x y = f y x
sub :: Int -> Int -> Int
sub a b = a - b
main = print (flip' sub 3 5)
