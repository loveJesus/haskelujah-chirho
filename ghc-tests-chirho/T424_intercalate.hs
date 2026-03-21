-- TEST: compile_and_run
-- EXPECTED: 1-2-3-4-5
module Main where
intercalate _ [] = ""; intercalate _ [x] = show x
intercalate sep (x:xs) = show x ++ sep ++ intercalate sep xs
main = putStrLn (intercalate "-" [1,2,3,4,5])
