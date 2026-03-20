-- TEST: compile_and_run
-- EXPECTED: 20\n-1
module Main where
lookup' :: Int -> [(Int, Int)] -> Maybe Int
lookup' _ [] = Nothing
lookup' key ((k, v):rest) = if key == k then Just v else lookup' key rest
fromMaybe :: Int -> Maybe Int -> Int
fromMaybe def Nothing = def
fromMaybe _ (Just x) = x
main :: IO ()
main = do
  let table = [(1, 10), (2, 20), (3, 30)]
  print (fromMaybe 0 (lookup' 2 table))
  print (fromMaybe (-1) (lookup' 5 table))
