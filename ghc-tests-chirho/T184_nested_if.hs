-- TEST: compile_and_run
-- EXPECTED: big\nmedium\nsmall
module Main where
classify :: Int -> String
classify n = if n > 100 then "big" else if n > 10 then "medium" else "small"
main = do { putStrLn (classify 200); putStrLn (classify 50); putStrLn (classify 5) }
