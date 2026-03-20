-- TEST: compile_and_run
-- EXPECTED: empty\n42\n1, 2, 3
module Main where
showNums :: [Int] -> String
showNums [] = "empty"
showNums (x:[]) = show x
showNums (x:xs) = show x ++ ", " ++ showNums xs
main :: IO ()
main = do
  putStrLn (showNums [])
  putStrLn (showNums [42])
  putStrLn (showNums [1, 2, 3])
