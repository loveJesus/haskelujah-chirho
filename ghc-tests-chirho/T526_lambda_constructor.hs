-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 42\nOk 84\nErr parse error
module Main where
data R = V Int | E
f :: R -> (Int -> R) -> R
f (V a) g = g a
f E _ = E
data Result a = Ok a | Err String
bindResult :: Result a -> (a -> Result b) -> Result b
bindResult (Err e) _ = Err e
bindResult (Ok a) g = g a
parseInt :: String -> Result Int
parseInt s = case s of
  "42" -> Ok 42
  "0"  -> Ok 0
  _    -> Err "parse error"
showR :: Result Int -> String
showR (Ok n) = "Ok " ++ show n
showR (Err e) = "Err " ++ e
main :: IO ()
main = do
  case f (V 21) (\n -> V (n * 2)) of
    V n -> print n
    E -> putStrLn "err"
  putStrLn (showR (bindResult (parseInt "42") (\n -> Ok (n * 2))))
  putStrLn (showR (parseInt "hello"))
