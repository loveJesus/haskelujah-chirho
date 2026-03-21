-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Just 40\nNothing\nJust 10\n99
module Main where
safeLookup :: Int -> [a] -> Maybe a
safeLookup _ [] = Nothing
safeLookup 0 (x:_) = Just x
safeLookup n (_:xs) = safeLookup (n-1) xs
safeDiv :: Int -> Int -> Maybe Int
safeDiv _ 0 = Nothing
safeDiv a b = Just (div a b)
andThen :: Maybe a -> (a -> Maybe b) -> Maybe b
andThen Nothing _ = Nothing
andThen (Just x) f = f x
withDefault :: a -> Maybe a -> a
withDefault def Nothing = def
withDefault _ (Just x) = x
showMaybe :: Maybe Int -> String
showMaybe Nothing = "Nothing"
showMaybe (Just x) = "Just " ++ show x
main :: IO ()
main = do
  putStrLn (showMaybe (safeLookup 2 [40,41,42,43]))
  putStrLn (showMaybe (safeDiv 10 0))
  putStrLn (showMaybe (andThen (safeDiv 100 10) (\x -> Just x)))
  print (withDefault 99 Nothing)
