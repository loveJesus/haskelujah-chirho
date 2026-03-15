-- TEST: compile_and_run
-- EXPECT_OUTPUT: two
-- Case expressions
main = putStrLn (descr 2)

descr :: Int -> String
descr n = case n of
  1 -> "one"
  2 -> "two"
  3 -> "three"
  _ -> "other"
