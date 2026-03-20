-- TEST: compile_and_run
-- EXPECTED: Haskelujah: 180 tests, all green!
module Main where
main :: IO ()
main = putStrLn ("Haskelujah: " ++ show 180 ++ " tests, all green!")
