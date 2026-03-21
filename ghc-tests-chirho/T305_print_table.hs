-- TEST: compile_and_run
-- EXPECTED: 1 -> 1\n2 -> 4\n3 -> 9\n4 -> 16\n5 -> 25
module Main where
printTable :: Int -> Int -> IO ()
printTable n limit
  | n > limit = return ()
  | otherwise = do
      putStrLn (show n ++ " -> " ++ show (n * n))
      printTable (n + 1) limit
main :: IO ()
main = printTable 1 5
