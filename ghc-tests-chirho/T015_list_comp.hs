-- TEST: compile_and_run
-- EXPECT_OUTPUT: [2,4,6,8,10]
-- List comprehension
main = print [x * 2 | x <- [1..5]]
