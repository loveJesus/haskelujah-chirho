-- TEST: compile_and_run
-- EXPECTED: A\nB\nC\nF
module Main where
grade :: Int -> String
grade score
  | score >= 90 = "A"
  | score >= 80 = "B"
  | score >= 70 = "C"
  | otherwise = "F"
main :: IO ()
main = do
  putStrLn (grade 95)
  putStrLn (grade 85)
  putStrLn (grade 75)
  putStrLn (grade 50)
