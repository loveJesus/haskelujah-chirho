-- TEST: compile_and_run
-- EXPECTED: 1\n4\n9\n16\n25
module Main where
loop :: Int -> Int -> IO ()
loop i limit
  | i > limit = return ()
  | otherwise = do { print (i * i); loop (i + 1) limit }
main = loop 1 5
