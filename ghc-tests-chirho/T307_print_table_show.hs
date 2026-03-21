-- TEST: compile_and_run
-- EXPECTED: 1 * 1 = 1\n2 * 2 = 4\n3 * 3 = 9
module Main where
printMulTable :: Int -> Int -> IO ()
printMulTable n limit
  | n > limit = return ()
  | otherwise = do
      putStrLn (show n ++ " * " ++ show n ++ " = " ++ show (n*n))
      printMulTable (n+1) limit
main = printMulTable 1 3
