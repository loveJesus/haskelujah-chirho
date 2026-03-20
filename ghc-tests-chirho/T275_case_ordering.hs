-- TEST: compile_and_run
-- EXPECTED: equal\nless\ngreater
module Main where
cmp a b | a == b = EQ | a < b = LT | otherwise = GT
showOrd EQ = "equal"; showOrd LT = "less"; showOrd GT = "greater"
main = do { putStrLn (showOrd (cmp 5 5)); putStrLn (showOrd (cmp 3 5)); putStrLn (showOrd (cmp 7 5)) }
