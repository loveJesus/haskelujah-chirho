-- TEST: compile_and_run
-- EXPECTED: Haskelujah Chirho: 160 curated tests!
module Main where
main :: IO ()
main = putStrLn ("Haskelujah Chirho: " ++ show 160 ++ " curated tests!")
