-- TEST: compile_and_run
-- EXPECTED: count: 5\nsum: 15\nmean: 3
module Main where
mySum [] = 0; mySum (x:xs) = x + mySum xs
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main :: IO ()
main = do
  let xs = [1,2,3,4,5]
  putStrLn ("count: " ++ show (myLen xs))
  putStrLn ("sum: " ++ show (mySum xs))
  putStrLn ("mean: " ++ show (mySum xs `div` myLen xs))
