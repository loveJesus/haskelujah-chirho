-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
classify :: Int -> Int
classify limit = count 2 0
  where
    count i acc
      | i >= limit = acc
      | check i = count (i + 1) (acc + 1)
      | otherwise = count (i + 1) acc
    check i = trial i 2
    trial i d
      | d * d > i = True
      | i `mod` d == 0 = False
      | otherwise = trial i (d + 1)
main :: IO ()
main = print (classify 50)
