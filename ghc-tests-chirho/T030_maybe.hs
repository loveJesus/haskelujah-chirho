-- TEST: compile_and_run
-- EXPECTED: 42\n0
module Main where
fromMaybe :: Int -> Maybe Int -> Int
fromMaybe def Nothing = def
fromMaybe _ (Just x) = x
main :: IO ()
main = do
  print (fromMaybe 0 (Just 42))
  print (fromMaybe 0 Nothing)
