-- TEST: compile_and_run
-- EXPECTED: [3 + 4 = 7]
module Main where
main = putStrLn ("[" ++ show 3 ++ " + " ++ show 4 ++ " = " ++ show 7 ++ "]")
