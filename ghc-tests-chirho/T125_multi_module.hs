-- TEST: compile
-- Multi-module import should compile
module Main where
import Prelude
factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)
main :: IO ()
main = print (factorial 10)
