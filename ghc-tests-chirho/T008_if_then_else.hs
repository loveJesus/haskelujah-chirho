-- TEST: compile_and_run
-- EXPECT_OUTPUT: even
-- If-then-else
main = putStrLn (check 42)

check :: Int -> String
check n = if n `mod` 2 == 0 then "even" else "odd"
