-- TEST: compile_and_run
-- EXPECTED: 13
module Main where
caesarShift :: Int -> Int -> Int
caesarShift shift c = ((c - 65 + shift) `mod` 26) + 65
main = print (caesarShift 13 65 - 65)
