-- TEST: compile_and_run
-- EXPECTED: 24
module Main where
powerMod :: Int -> Int -> Int -> Int
powerMod _ 0 _ = 1
powerMod b e m = (b * powerMod b (e-1) m) `mod` m
main = print (powerMod 2 10 1000)
