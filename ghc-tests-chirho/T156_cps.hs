-- TEST: compile_and_run
-- EXPECTED: 7\n30\n14
module Main where
addCPS :: Int -> Int -> (Int -> Int) -> Int
addCPS x y k = k (x + y)
mulCPS :: Int -> Int -> (Int -> Int) -> Int
mulCPS x y k = k (x * y)
identity :: Int -> Int
identity x = x
main :: IO ()
main = do
  print (addCPS 3 4 identity)
  print (mulCPS 5 6 identity)
  print (addCPS 3 4 (\s -> mulCPS s 2 identity))
