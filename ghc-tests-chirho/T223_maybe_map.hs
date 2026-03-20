-- TEST: compile_and_run
-- EXPECTED: 43\n0
module Main where
mapMaybe :: (Int -> Int) -> Maybe Int -> Maybe Int
mapMaybe _ Nothing = Nothing
mapMaybe f (Just x) = Just (f x)
fromMaybe def Nothing = def; fromMaybe _ (Just x) = x
main = do
  print (fromMaybe 0 (mapMaybe (+1) (Just 42)))
  print (fromMaybe 0 (mapMaybe (+1) Nothing))
