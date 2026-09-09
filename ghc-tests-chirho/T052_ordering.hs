-- TEST: compile_and_run
-- EXPECTED: -1\n0\n1
module Main where
import Prelude hiding (Ordering(..))
data Ordering = LT | EQ | GT
myCompare :: Int -> Int -> Ordering
myCompare a b
  | a < b = LT
  | a == b = EQ
  | otherwise = GT
ordToInt :: Ordering -> Int
ordToInt LT = -1
ordToInt EQ = 0
ordToInt GT = 1
main :: IO ()
main = do
  print (ordToInt (myCompare 3 5))
  print (ordToInt (myCompare 5 5))
  print (ordToInt (myCompare 7 5))
