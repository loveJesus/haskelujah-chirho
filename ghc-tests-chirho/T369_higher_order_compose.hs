-- TEST: compile_and_run
-- EXPECTED: 50
module Main where
compose f g x = f (g x)
add10 x = x + 10
mul2 x = x * 2
main = print (compose add10 mul2 20)
