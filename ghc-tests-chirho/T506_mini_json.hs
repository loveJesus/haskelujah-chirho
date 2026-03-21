-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: JNum 42\nJStr hello\nJArr [JNum 1,JNum 2,JNum 3]\nJObj [(name,JStr Alice),(age,JNum 30)]
module Main where

-- Mini JSON-like AST — tests ADTs, Show, recursive types, map
data JValue = JNum Int
            | JStr String
            | JBool Bool
            | JNull
            | JArr [JValue]
            | JObj [(String, JValue)]

showJValue :: JValue -> String
showJValue (JNum n) = "JNum " ++ show n
showJValue (JStr s) = "JStr " ++ s
showJValue (JBool b) = "JBool " ++ show b
showJValue JNull = "JNull"
showJValue (JArr vs) = "JArr [" ++ intercalateJ "," (myMap showJValue vs) ++ "]"
showJValue (JObj kvs) = "JObj [" ++ intercalateJ "," (myMap showPair kvs) ++ "]"

showPair :: (String, JValue) -> String
showPair (k, v) = "(" ++ k ++ "," ++ showJValue v ++ ")"

intercalateJ :: String -> [String] -> String
intercalateJ _ [] = ""
intercalateJ _ [x] = x
intercalateJ sep (x:xs) = x ++ sep ++ intercalateJ sep xs

myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs

main :: IO ()
main = do
  putStrLn (showJValue (JNum 42))
  putStrLn (showJValue (JStr "hello"))
  putStrLn (showJValue (JArr [JNum 1, JNum 2, JNum 3]))
  putStrLn (showJValue (JObj [("name", JStr "Alice"), ("age", JNum 30)]))
