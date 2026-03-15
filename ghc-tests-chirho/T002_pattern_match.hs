-- TEST: compile_and_run
-- EXPECT_OUTPUT: 120
-- Pattern matching and recursion
main = print (fac 5)

fac :: Int -> Int
fac 0 = 1
fac n = n * fac (n - 1)
