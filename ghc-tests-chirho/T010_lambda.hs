-- TEST: compile_and_run
-- EXPECT_OUTPUT: 42
-- Lambda expressions
main = print ((\x -> x * 2) 21)
