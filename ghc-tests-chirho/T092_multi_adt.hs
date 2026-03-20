-- TEST: compile_and_run
-- EXPECTED: 1\n0\n2\n3
module Main where
data Direction = North | South | East | West
dirToInt :: Direction -> Int
dirToInt North = 0
dirToInt South = 1
dirToInt East = 2
dirToInt West = 3
opposite :: Direction -> Direction
opposite North = South
opposite South = North
opposite East = West
opposite West = East
main :: IO ()
main = do
  print (dirToInt (opposite North))
  print (dirToInt (opposite South))
  print (dirToInt East)
  print (dirToInt West)
