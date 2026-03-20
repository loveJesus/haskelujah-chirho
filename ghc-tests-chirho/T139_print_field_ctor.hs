-- TEST: compile_and_run
-- EXPECTED: MkPair 3 4
module Main where
data Pair = MkPair Int Int deriving (Show)
main :: IO ()
main = print (MkPair 3 4)
