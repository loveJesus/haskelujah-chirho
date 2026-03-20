-- TEST: compile_and_run
-- EXPECTED: GT\nLT\nEQ
module Main where
data Prio = Low | Med | High deriving (Eq, Ord)
main :: IO ()
main = do
  putStrLn (show (compare High Low))
  putStrLn (show (compare Low High))
  putStrLn (show (compare Med Med))
