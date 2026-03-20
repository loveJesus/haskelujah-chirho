-- TEST: compile_and_run
-- EXPECTED: 3...\n2...\n1...\nGo!
module Main where
countDown :: Int -> IO ()
countDown 0 = putStrLn "Go!"
countDown n = do
  putStrLn (show n ++ "...")
  countDown (n - 1)
main :: IO ()
main = countDown 3
