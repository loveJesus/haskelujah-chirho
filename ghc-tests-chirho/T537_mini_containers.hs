-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: fromList [(1,hello),(2,world),(3,!)]\nJust world\nNothing\n3\n[1,2,3]\n[hello,world,!]
module Main where
-- Mini BST-based Map (like Data.Map from containers)
data Map k v = Tip | Bin Int k v (Map k v) (Map k v)

empty :: Map k v
empty = Tip

singleton :: k -> v -> Map k v
singleton k v = Bin 1 k v Tip Tip

size :: Map k v -> Int
size Tip = 0
size (Bin s _ _ _ _) = s

insert :: Int -> v -> Map Int v -> Map Int v
insert k v Tip = singleton k v
insert k v (Bin s k2 v2 l r)
  | k < k2 = Bin (s+1) k2 v2 (insert k v l) r
  | k > k2 = Bin (s+1) k2 v2 l (insert k v r)
  | otherwise = Bin s k v l r

lookupMap :: Int -> Map Int v -> Maybe v
lookupMap _ Tip = Nothing
lookupMap k (Bin _ k2 v l r)
  | k < k2 = lookupMap k l
  | k > k2 = lookupMap k r
  | otherwise = Just v

keys :: Map k v -> [k]
keys Tip = []
keys (Bin _ k _ l r) = keys l ++ [k] ++ keys r

elems :: Map k v -> [v]
elems Tip = []
elems (Bin _ _ v l r) = elems l ++ [v] ++ elems r

fromList :: [(Int, v)] -> Map Int v
fromList [] = empty
fromList ((k,v):rest) = insert k v (fromList rest)

showMap :: Map Int String -> String
showMap m = "fromList " ++ show (toAssocList m)

toAssocList :: Map k v -> [(k, v)]
toAssocList Tip = []
toAssocList (Bin _ k v l r) = toAssocList l ++ [(k, v)] ++ toAssocList r

showMaybe :: Maybe String -> String
showMaybe Nothing = "Nothing"
showMaybe (Just s) = "Just " ++ s

main :: IO ()
main = do
  let m = fromList [(2, "world"), (1, "hello"), (3, "!")]
  putStrLn (showMap m)
  putStrLn (showMaybe (lookupMap 2 m))
  putStrLn (showMaybe (lookupMap 5 m))
  print (size m)
  print (keys m)
  print (elems m)
