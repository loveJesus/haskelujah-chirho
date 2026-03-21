-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
enqueue :: Int -> [Int] -> [Int]
enqueue x [] = [x]
enqueue x (y:ys) = y : enqueue x ys
myLen :: [Int] -> Int
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
main :: IO ()
main = do
  let q = enqueue 3 (enqueue 2 (enqueue 1 []))
  print (myLen q)
