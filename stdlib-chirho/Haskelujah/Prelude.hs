-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Extended Prelude with batteries included.
-- Import this instead of Prelude for a richer default environment.
module Haskelujah.Prelude (
    -- Re-exports from Prelude
    module Prelude,
    -- Text utilities
    trim,
    words',
    unwords',
    -- List utilities  
    groupBy',
    chunksOf,
    nub',
) where

-- | Trim leading and trailing whitespace.
trim :: String -> String
trim = reverse . dropWhile isSpace . reverse . dropWhile isSpace
  where isSpace c = c == ' ' || c == '\n' || c == '\t' || c == '\r'

-- | Split string by whitespace (like words but handles multiple spaces).
words' :: String -> [String]
words' [] = []
words' s = case dropWhile isSpace s of
    [] -> []
    s' -> let (w, rest) = break isSpace s' in w : words' rest
  where isSpace c = c == ' ' || c == '\n' || c == '\t'

-- | Join strings with spaces.
unwords' :: [String] -> String
unwords' [] = ""
unwords' [x] = x
unwords' (x:xs) = x ++ " " ++ unwords' xs

-- | Group consecutive elements by a predicate.
groupBy' :: (a -> a -> Bool) -> [a] -> [[a]]
groupBy' _ [] = []
groupBy' eq (x:xs) = let (ys, zs) = span (eq x) xs in (x:ys) : groupBy' eq zs

-- | Split a list into chunks of size n.
chunksOf :: Int -> [a] -> [[a]]
chunksOf _ [] = []
chunksOf n xs = take n xs : chunksOf n (drop n xs)

-- | Remove duplicates (O(n^2) but no Ord constraint).
nub' :: Eq a => [a] -> [a]
nub' [] = []
nub' (x:xs) = x : nub' (filter (/= x) xs)
