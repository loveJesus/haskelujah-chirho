-- TEST: compile
-- print for constructor with fields
module Main where
data Pair = MkPair Int Int deriving (Show)
main :: IO ()
main = print (MkPair 3 4)
