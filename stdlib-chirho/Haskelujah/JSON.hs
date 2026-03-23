-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in JSON support. No package install needed.
--
-- @
-- import Haskelujah.JSON
-- main = putStrLn (encode (object [("name", String "hello")]))
-- @
module Haskelujah.JSON (
    Value(..),
    encode,
    object,
    array,
    (.=),
) where

data Value
    = Object [(String, Value)]
    | Array [Value]
    | String String
    | Number Double
    | Bool Bool
    | Null
    deriving (Show, Eq)

-- | Encode a Value to a JSON string.
encode :: Value -> String
encode (Object pairs) = "{" ++ intercalate "," (map encodePair pairs) ++ "}"
  where encodePair (k, v) = "\"" ++ escapeString k ++ "\":" ++ encode v
encode (Array vals) = "[" ++ intercalate "," (map encode vals) ++ "]"
encode (String s) = "\"" ++ escapeString s ++ "\""
encode (Number n) = show n
encode (Bool True) = "true"
encode (Bool False) = "false"
encode Null = "null"

escapeString :: String -> String
escapeString [] = []
escapeString ('"':cs) = '\\' : '"' : escapeString cs
escapeString ('\\':cs) = '\\' : '\\' : escapeString cs
escapeString ('\n':cs) = '\\' : 'n' : escapeString cs
escapeString ('\t':cs) = '\\' : 't' : escapeString cs
escapeString (c:cs) = c : escapeString cs

intercalate :: String -> [String] -> String
intercalate _ [] = ""
intercalate _ [x] = x
intercalate sep (x:xs) = x ++ sep ++ intercalate sep xs

-- | Create a JSON object from key-value pairs.
object :: [(String, Value)] -> Value
object = Object

-- | Create a JSON array from a list of values.
array :: [Value] -> Value
array = Array

-- | Create a key-value pair for use with 'object'.
(.=) :: String -> Value -> (String, Value)
k .= v = (k, v)
