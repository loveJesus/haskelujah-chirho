-- TEST: compile_and_run
-- EXPECT_OUTPUT: 12
-- Operator sections
main = print (foldr (+) 0 (map (*2) [1,2,3]))
