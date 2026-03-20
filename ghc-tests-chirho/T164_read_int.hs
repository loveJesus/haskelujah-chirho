-- TEST: compile_and_run
-- EXPECTED: 50
module Main where
main :: IO ()
main = print ((read "42" :: Int) + 8)
