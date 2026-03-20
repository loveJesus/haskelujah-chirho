-- TEST: compile_and_run
-- EXPECTED: 3...\n2...\n1...\nBlastoff!
module Main where
countdown :: Int -> IO ()
countdown 0 = putStrLn "Blastoff!"
countdown n = do
  putStrLn (show n ++ "...")
  countdown (n - 1)
main :: IO ()
main = countdown 3
