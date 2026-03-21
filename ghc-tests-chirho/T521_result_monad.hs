-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Ok 42\nOk 84\nErr not a number
module Main where
data Result a = Ok a | Err String
bindR :: Result a -> (a -> Result b) -> Result b
bindR (Err e) _ = Err e
bindR (Ok a) f = f a
mapR :: (a -> b) -> Result a -> Result b
mapR _ (Err e) = Err e
mapR f (Ok a) = Ok (f a)
parseInt :: String -> Result Int
parseInt s = case s of
  "42" -> Ok 42
  "0"  -> Ok 0
  _    -> Err ("not a number")
showResult :: Result Int -> String
showResult (Ok n) = "Ok " ++ show n
showResult (Err e) = "Err " ++ e
doubleOk :: Int -> Result Int
doubleOk n = Ok (n * 2)
main :: IO ()
main = do
  putStrLn (showResult (parseInt "42"))
  putStrLn (showResult (bindR (parseInt "42") doubleOk))
  putStrLn (showResult (parseInt "hello"))
