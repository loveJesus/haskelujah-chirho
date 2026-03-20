-- TEST: compile_and_run
-- EXPECTED: 5\n3
module Main where
countPosNeg :: [Int] -> (Int, Int)
countPosNeg xs = go xs 0 0
  where
    go [] pos neg = (pos, neg)
    go (x:rest) pos neg
      | x > 0 = go rest (pos + 1) neg
      | x < 0 = go rest pos (neg + 1)
      | otherwise = go rest pos neg
main :: IO ()
main = do
  let (p, n) = countPosNeg [3, -1, 4, -1, 5, -9, 2, 6]
  print p
  print n
