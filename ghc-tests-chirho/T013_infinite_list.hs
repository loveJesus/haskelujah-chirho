-- TEST: compile_and_run
-- EXPECT_OUTPUT: 15
-- Infinite lists with take
main = print (sum (take 5 [1..]))
