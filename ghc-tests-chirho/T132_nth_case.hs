-- TEST: compile_and_run
-- EXPECTED: 10\n20\n30
module Main where
nth :: [Int] -> Int -> Int
nth xs n = case xs of
  [] -> -1
  (x:rest) -> if n == 0 then x else nth rest (n - 1)
main :: IO ()
main = do
  print (nth [10, 20, 30] 0)
  print (nth [10, 20, 30] 1)
  print (nth [10, 20, 30] 2)
