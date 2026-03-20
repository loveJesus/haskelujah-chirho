-- TEST: compile
-- Derived Ord on enum type
module Main where
data Prio = Low | Med | High deriving (Eq, Ord)
main :: IO ()
main = putStrLn (show (compare High Low))
