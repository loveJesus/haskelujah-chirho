-- TEST: compile_and_run
-- EXPECTED: 3\n2\n1\nafter
module Main where
helper :: Int -> IO ()
helper 0 = return ()
helper n = do
  print n
  helper (n - 1)
main :: IO ()
main = do
  helper 3
  putStrLn "after"
