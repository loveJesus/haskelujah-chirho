{-# LANGUAGE CPP #-}
-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Just 1\nNothing\nJust [2,3]\nJust 3\nJust [1,2]\nJust 30\nJust 1\nJust 9
module Main where
headMay :: [a] -> Maybe a
headMay [] = Nothing
headMay (x:_) = Just x
tailMay :: [a] -> Maybe [a]
tailMay [] = Nothing
tailMay (_:xs) = Just xs
lastMay :: [a] -> Maybe a
lastMay [] = Nothing
lastMay [x] = Just x
lastMay (_:xs) = lastMay xs
initMay :: [a] -> Maybe [a]
initMay [] = Nothing
initMay [_] = Just []
initMay (x:xs) = case initMay xs of
  Just ys -> Just (x:ys)
  Nothing -> Just []
atMay :: [a] -> Int -> Maybe a
atMay [] _ = Nothing
atMay (x:xs) n = if n == 0 then Just x else atMay xs (n-1)
minimumMay :: [Int] -> Maybe Int
minimumMay [] = Nothing
minimumMay (x:xs) = Just (go x xs)
  where go m [] = m
        go m (y:ys) = go (if y < m then y else m) ys
maximumMay :: [Int] -> Maybe Int
maximumMay [] = Nothing
maximumMay (x:xs) = Just (go x xs)
  where go m [] = m
        go m (y:ys) = go (if y > m then y else m) ys
showMaybe :: Maybe Int -> String
showMaybe Nothing = "Nothing"
showMaybe (Just x) = "Just " ++ show x
showMaybeList :: Maybe [Int] -> String
showMaybeList Nothing = "Nothing"
showMaybeList (Just xs) = "Just " ++ show xs
main :: IO ()
main = do
  putStrLn (showMaybe (headMay [1,2,3]))
  putStrLn (showMaybe (headMay ([] :: [Int])))
  putStrLn (showMaybeList (tailMay [1,2,3]))
  putStrLn (showMaybe (lastMay [1,2,3]))
  putStrLn (showMaybeList (initMay [1,2,3]))
  putStrLn (showMaybe (atMay [10,20,30,40] 2))
  putStrLn (showMaybe (minimumMay [3,1,4,1,5,9]))
  putStrLn (showMaybe (maximumMay [3,1,4,1,5,9]))
