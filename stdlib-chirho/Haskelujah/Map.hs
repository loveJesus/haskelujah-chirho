-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in association-list Map. No package install needed.
-- For production use, import Data.Map from containers.
module Haskelujah.Map (
    Map,
    empty,
    singleton,
    fromList,
    toList,
    insert,
    lookup,
    delete,
    member,
    size,
    keys,
    elems,
    mapValues,
    filterMap,
    unionWith,
) where

import Prelude hiding (lookup)

-- | Simple association-list map.
type Map k v = [(k, v)]

empty :: Map k v
empty = []

singleton :: k -> v -> Map k v
singleton k v = [(k, v)]

fromList :: Eq k => [(k, v)] -> Map k v
fromList = foldl (\m (k, v) -> insert k v m) empty

toList :: Map k v -> [(k, v)]
toList = id

insert :: Eq k => k -> v -> Map k v -> Map k v
insert k v [] = [(k, v)]
insert k v ((k', v'):rest)
    | k == k'   = (k, v) : rest
    | otherwise  = (k', v') : insert k v rest

lookup :: Eq k => k -> Map k v -> Maybe v
lookup _ [] = Nothing
lookup k ((k', v'):rest)
    | k == k'   = Just v'
    | otherwise  = lookup k rest

delete :: Eq k => k -> Map k v -> Map k v
delete _ [] = []
delete k ((k', v'):rest)
    | k == k'   = rest
    | otherwise  = (k', v') : delete k rest

member :: Eq k => k -> Map k v -> Bool
member k m = case lookup k m of
    Just _  -> True
    Nothing -> False

size :: Map k v -> Int
size = length

keys :: Map k v -> [k]
keys = map fst

elems :: Map k v -> [v]
elems = map snd

mapValues :: (v -> w) -> Map k v -> Map k w
mapValues f = map (\(k, v) -> (k, f v))

filterMap :: (k -> v -> Bool) -> Map k v -> Map k v
filterMap f = filter (uncurry f)

unionWith :: Eq k => (v -> v -> v) -> Map k v -> Map k v -> Map k v
unionWith f m1 m2 = foldl go m1 m2
  where go acc (k, v) = case lookup k acc of
          Just v' -> insert k (f v' v) acc
          Nothing -> insert k v acc
