-- TEST: compile_and_run
-- EXPECTED: [1, 2, 3, 4, 5]
module Main where
showList_ [] = ""; showList_ [x] = show x; showList_ (x:xs) = show x ++ ", " ++ showList_ xs
main = putStrLn ("[" ++ showList_ [1,2,3,4,5] ++ "]")
