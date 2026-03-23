-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in Set as sorted list. No package install needed.
module Haskelujah.Set (
    Set,
    empty,
    singleton,
    fromList,
    toList,
    insert,
    member,
    delete,
    size,
    union,
    intersection,
    difference,
) where

-- | A set as a sorted deduplicated list.
type Set a = [a]

empty :: Set a
empty = []

singleton :: a -> Set a
singleton x = [x]

fromList :: (Eq a, Ord a) => [a] -> Set a
fromList = nub . sort
  where
    nub [] = []
    nub [x] = [x]
    nub (x:y:rest)
        | x == y    = nub (y:rest)
        | otherwise  = x : nub (y:rest)
    sort [] = []
    sort (x:xs) = sort [y | y <- xs, y < x] ++ [x] ++ sort [y | y <- xs, y >= x]

toList :: Set a -> [a]
toList = id

insert :: (Eq a, Ord a) => a -> Set a -> Set a
insert x [] = [x]
insert x (y:ys)
    | x == y    = y : ys
    | x < y     = x : y : ys
    | otherwise  = y : insert x ys

member :: Eq a => a -> Set a -> Bool
member _ [] = False
member x (y:ys)
    | x == y    = True
    | otherwise  = member x ys

delete :: Eq a => a -> Set a -> Set a
delete _ [] = []
delete x (y:ys)
    | x == y    = ys
    | otherwise  = y : delete x ys

size :: Set a -> Int
size = length

union :: (Eq a, Ord a) => Set a -> Set a -> Set a
union xs ys = fromList (xs ++ ys)

intersection :: Eq a => Set a -> Set a -> Set a
intersection xs ys = filter (\x -> member x ys) xs

difference :: Eq a => Set a -> Set a -> Set a
difference xs ys = filter (\x -> not (member x ys)) xs
