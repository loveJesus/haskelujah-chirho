-- TEST: compile_and_run
-- EXPECTED: 3\n2
module Main where
divMod_ :: Int -> Int -> (Int, Int)
divMod_ a b = (a `div` b, a `mod` b)
main :: IO ()
main = do
  let (q, r) = divMod_ 17 5
  print q
  print r
