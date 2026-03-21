-- TEST: compile_and_run
-- EXPECTED: 2
module Main where
flip_ f x y = f y x
sub a b = a - b
main = print (flip_ sub 3 5)
