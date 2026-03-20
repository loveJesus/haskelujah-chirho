-- TEST: compile
-- String case expression
module Main where
classify :: String -> String
classify s = case s of
  "hello" -> "greeting"
  "bye" -> "farewell"
  _ -> "unknown"
main :: IO ()
main = putStrLn (classify "hello")
