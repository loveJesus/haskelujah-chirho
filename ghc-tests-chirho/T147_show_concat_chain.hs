-- TEST: compile_and_run
-- EXPECTED: x=10, y=20, sum=30
module Main where
main :: IO ()
main = do
  let x = 10
      y = 20
  putStrLn ("x=" ++ show x ++ ", y=" ++ show y ++ ", sum=" ++ show (x + y))
