-- TEST: compile_and_run
-- EXPECTED: 0\n1\n2\n3
module Main where
data Direction = North | South | East | West
opposite :: Direction -> Direction
opposite d = case d of
  North -> South
  South -> North
  East -> West
  West -> East
dirToInt :: Direction -> Int
dirToInt d = case d of
  North -> 0
  South -> 1
  East -> 2
  West -> 3
main :: IO ()
main = do
  print (dirToInt North)
  print (dirToInt (opposite North))
  print (dirToInt East)
  print (dirToInt (opposite East))
