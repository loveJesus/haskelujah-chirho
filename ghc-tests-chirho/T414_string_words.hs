-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main = print (myLen ["hello", "world", "foo"])
