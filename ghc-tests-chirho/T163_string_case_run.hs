-- TEST: compile_and_run
-- EXPECTED: greeting\nfarwell\nunknown
module Main where
classify :: String -> String
classify s = case s of
  "hello" -> "greeting"
  "bye" -> "farwell"
  _ -> "unknown"
main :: IO ()
main = do
  putStrLn (classify "hello")
  putStrLn (classify "bye")
  putStrLn (classify "xyz")
