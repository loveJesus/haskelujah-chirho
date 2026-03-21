-- TEST: compile_and_run
-- EXPECTED: even\nodd
module Main where
classify :: Int -> String
classify n = label
  where label = if r == 0 then "even" else "odd"
        r = n `mod` 2
main :: IO ()
main = do { putStrLn (classify 42); putStrLn (classify 7) }
