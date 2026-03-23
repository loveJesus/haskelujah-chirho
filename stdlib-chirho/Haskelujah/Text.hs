-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in text utilities. No package install needed.
module Haskelujah.Text (
    -- Case conversion
    toLower,
    toUpper,
    capitalize,
    -- Searching
    contains,
    startsWith,
    endsWith,
    -- Splitting
    splitOn,
    lines',
    unlines',
    -- Padding
    padLeft,
    padRight,
    center,
) where

-- | Convert to lowercase.
toLower :: String -> String
toLower = map toLowerChar
  where toLowerChar c
          | c >= 'A' && c <= 'Z' = toEnum (fromEnum c + 32)
          | otherwise = c

-- | Convert to uppercase.
toUpper :: String -> String
toUpper = map toUpperChar
  where toUpperChar c
          | c >= 'a' && c <= 'z' = toEnum (fromEnum c - 32)
          | otherwise = c

-- | Capitalize first letter.
capitalize :: String -> String
capitalize [] = []
capitalize (c:cs) = toUpperChar c : cs
  where toUpperChar ch
          | ch >= 'a' && ch <= 'z' = toEnum (fromEnum ch - 32)
          | otherwise = ch

-- | Check if a string contains a substring.
contains :: String -> String -> Bool
contains _ [] = True
contains [] _ = False
contains haystack needle = take (length needle) haystack == needle
    || contains (tail haystack) needle

-- | Check if a string starts with a prefix.
startsWith :: String -> String -> Bool
startsWith str prefix = take (length prefix) str == prefix

-- | Check if a string ends with a suffix.
endsWith :: String -> String -> Bool
endsWith str suffix = drop (length str - length suffix) str == suffix

-- | Split a string on a delimiter.
splitOn :: String -> String -> [String]
splitOn _ [] = [""]
splitOn sep str = go str
  where
    sepLen = length sep
    go [] = [""]
    go s
      | take sepLen s == sep = "" : go (drop sepLen s)
      | otherwise = case go (tail s) of
          (first:rest) -> (head s : first) : rest
          [] -> [[head s]]

-- | Split on newlines.
lines' :: String -> [String]
lines' = splitOn "\n"

-- | Join with newlines.
unlines' :: [String] -> String
unlines' [] = ""
unlines' [x] = x
unlines' (x:xs) = x ++ "\n" ++ unlines' xs

-- | Pad string on the left to width n.
padLeft :: Int -> Char -> String -> String
padLeft n c s = replicate (max 0 (n - length s)) c ++ s

-- | Pad string on the right to width n.
padRight :: Int -> Char -> String -> String
padRight n c s = s ++ replicate (max 0 (n - length s)) c

-- | Center string in width n.
center :: Int -> Char -> String -> String
center n c s =
    let pad = max 0 (n - length s)
        left = pad `div` 2
        right = pad - left
    in replicate left c ++ s ++ replicate right c
