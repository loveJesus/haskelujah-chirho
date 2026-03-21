-- TEST: compile_and_run
-- EXPECTED: start\n42\nTrue\nend
module Main where
main :: IO ()
main = do
  putStrLn "start"
  print 42
  print True
  putStrLn "end"
