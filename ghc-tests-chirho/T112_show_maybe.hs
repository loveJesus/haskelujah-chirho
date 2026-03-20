-- TEST: compile_and_run
-- EXPECTED: Just 42\nNothing
module Main where
showMaybe :: Maybe Int -> String
showMaybe Nothing = "Nothing"
showMaybe (Just x) = "Just " ++ show x
main :: IO ()
main = do
  putStrLn (showMaybe (Just 42))
  putStrLn (showMaybe Nothing)
