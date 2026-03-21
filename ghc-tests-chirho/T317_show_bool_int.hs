-- TEST: compile_and_run
-- EXPECTED: True\n42\nFalse
module Main where
main = do { putStrLn (show True); print 42; putStrLn (show False) }
