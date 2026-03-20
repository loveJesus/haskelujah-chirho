-- TEST: compile
-- read "42" :: Int
module Main where
main :: IO ()
main = print ((read "42" :: Int) + 8)
