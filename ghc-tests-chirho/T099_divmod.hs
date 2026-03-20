-- TEST: compile_and_run
-- EXPECTED: 3\n2
module Main where
divMod' :: Int -> Int -> (Int, Int)
divMod' a b = (a `div` b, a `mod` b)
main :: IO ()
main = do
  let (q, r) = divMod' 17 5
  print q
  print r
