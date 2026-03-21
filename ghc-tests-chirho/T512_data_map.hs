-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Just 1\nNothing\nJust 42\n3
module Main where
type Map k v = [(k, v)]

insertMap :: Int -> v -> Map Int v -> Map Int v
insertMap k v [] = [(k, v)]
insertMap k v ((k2,v2):rest) = if k == k2
  then (k, v) : rest
  else (k2, v2) : insertMap k v rest

lookupMap :: Int -> Map Int v -> Maybe v
lookupMap _ [] = Nothing
lookupMap k ((k2,v):rest) = if k == k2
  then Just v
  else lookupMap k rest

sizeMap :: Map k v -> Int
sizeMap [] = 0
sizeMap (_:xs) = 1 + sizeMap xs

showMaybe :: Maybe Int -> String
showMaybe Nothing = "Nothing"
showMaybe (Just x) = "Just " ++ show x

main :: IO ()
main = do
  let m0 = [] :: Map Int Int
  let m1 = insertMap 1 1 m0
  let m2 = insertMap 2 2 m1
  let m3 = insertMap 3 3 m2
  let m4 = insertMap 2 42 m3
  putStrLn (showMaybe (lookupMap 1 m4))
  putStrLn (showMaybe (lookupMap 9 m4))
  putStrLn (showMaybe (lookupMap 2 m4))
  print (sizeMap m4)
