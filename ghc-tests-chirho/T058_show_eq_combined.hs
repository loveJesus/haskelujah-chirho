-- TEST: compile
-- deriving Show + Eq on 7-constructor enum
module Main where
data Day = Mon | Tue | Wed | Thu | Fri | Sat | Sun deriving (Show, Eq)
isWeekend :: Day -> Bool
isWeekend Sat = True
isWeekend Sun = True
isWeekend _ = False
main :: IO ()
main = putStrLn (show Fri)
