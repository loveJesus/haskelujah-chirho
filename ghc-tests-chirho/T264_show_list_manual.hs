-- TEST: compile_and_run
-- EXPECTED: [1, 2, 3]
module Main where
showNums [] = ""; showNums [x] = show x; showNums (x:xs) = show x ++ ", " ++ showNums xs
main = putStrLn ("[" ++ showNums [1,2,3] ++ "]")
