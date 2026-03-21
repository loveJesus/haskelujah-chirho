-- TEST: compile_and_run
-- EXPECTED: 1\n4\n9\ndone
module Main where
printSquares :: Int -> Int -> IO ()
printSquares n limit
  | n > limit = return ()
  | otherwise = do
      print (n * n)
      printSquares (n + 1) limit
main :: IO ()
main = do
  printSquares 1 3
  putStrLn "done"
