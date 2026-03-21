-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
member :: Int -> [Int] -> Bool
member _ [] = False
member x (y:ys) = if x == y then True else member x ys
main :: IO ()
main = do
  print (bti (member 3 [1,2,3,4,5]))
  print (bti (member 6 [1,2,3,4,5]))
