-- TEST: compile_and_run
-- EXPECTED: hahaha
module Main where
repeatStr :: Int -> String -> String
repeatStr 0 _ = ""
repeatStr n s = s ++ repeatStr (n - 1) s
main :: IO ()
main = putStrLn (repeatStr 3 "ha")
