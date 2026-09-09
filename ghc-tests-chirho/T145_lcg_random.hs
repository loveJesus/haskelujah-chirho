-- TEST: compile_and_run
-- EXPECTED: 42\n27\n264
module Main where
lcg :: Int -> Int -> [Int]
lcg _ 0 = []
lcg seed n = (seed `mod` 1000) : lcg ((seed * 1103515245 + 12345) `mod` 2147483648) (n - 1)
nth :: [Int] -> Int -> Int
nth (x:_) 0 = x
nth (_:xs) n = nth xs (n - 1)
nth [] _ = -1
main :: IO ()
main = do
  let nums = lcg 42 10
  print (nth nums 0)
  print (nth nums 1)
  print (nth nums 2)
