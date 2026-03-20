-- TEST: compile_and_run
-- EXPECTED: x=10, y=20
module Main where
main :: IO ()
main = putStrLn ("x=" ++ show 10 ++ ", y=" ++ show 20)
