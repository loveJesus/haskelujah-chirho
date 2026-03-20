-- TEST: compile_and_run
-- EXPECTED: 105
module Main where
makeCounter :: Int -> Int -> Int
makeCounter start step = go start
  where go n = if n > 100 then n else go (n + step)
main :: IO ()
main = print (makeCounter 0 7)
