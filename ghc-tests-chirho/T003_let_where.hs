-- TEST: compile_and_run
-- EXPECT_OUTPUT: 42
-- Let and where bindings
main = print result
  where
    result = let x = 10
                 y = 32
             in x + y
