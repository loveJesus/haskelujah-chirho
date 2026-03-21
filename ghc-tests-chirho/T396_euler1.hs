-- TEST: compile_and_run
-- EXPECTED: 233168
module Main where
e1 l = go 0 0 where go a n = if n >= l then a else if n `mod` 3 == 0 || n `mod` 5 == 0 then go (a+n) (n+1) else go a (n+1)
main = print (e1 1000)
