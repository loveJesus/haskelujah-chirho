-- TEST: compile_and_run
-- EXPECTED: 5\n-1
module Main where
safeDiv :: Int -> Int -> Maybe Int
safeDiv _ 0 = Nothing
safeDiv a b = Just (a `div` b)
fromMaybe :: Int -> Maybe Int -> Int
fromMaybe def Nothing = def
fromMaybe _ (Just x) = x
main :: IO ()
main = do
  print (fromMaybe (-1) (safeDiv 10 2))
  print (fromMaybe (-1) (safeDiv 10 0))
