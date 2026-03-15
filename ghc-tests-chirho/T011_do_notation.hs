-- TEST: compile_and_run
-- EXPECT_OUTPUT: hello world
-- Do notation with IO
main = do
  putStr "hello "
  putStrLn "world"
