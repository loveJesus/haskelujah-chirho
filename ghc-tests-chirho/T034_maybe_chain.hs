-- TEST: compile_and_run
-- EXPECTED: 5\n-1\n-1
module Main where
safeDiv :: Int -> Int -> Maybe Int
safeDiv _ 0 = Nothing
safeDiv a b = Just (a `div` b)
fromMaybe :: Int -> Maybe Int -> Int
fromMaybe def Nothing = def
fromMaybe _ (Just x) = x
chain :: Int -> Int -> Int -> Int
chain a b c = fromMaybe (-1) (case safeDiv a b of
  Nothing -> Nothing
  Just ab -> safeDiv ab c)
main :: IO ()
main = do
  print (chain 100 5 4)
  print (chain 100 0 4)
  print (chain 100 5 0)
