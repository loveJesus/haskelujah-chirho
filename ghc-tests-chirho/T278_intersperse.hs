-- TEST: compile_and_run
-- EXPECTED: 1-2-3
module Main where
intersperse _ [] = ""; intersperse _ [x] = show x
intersperse sep (x:xs) = show x ++ sep ++ intersperse sep xs
main = putStrLn (intersperse "-" [1,2,3])
