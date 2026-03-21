-- TEST: compile_and_run
-- EXPECTED: A\nB\nC\nD\nF
module Main where
grade s | s >= 90 = "A" | s >= 80 = "B" | s >= 70 = "C" | s >= 60 = "D" | otherwise = "F"
main = do { putStrLn (grade 95); putStrLn (grade 85); putStrLn (grade 75); putStrLn (grade 65); putStrLn (grade 50) }
