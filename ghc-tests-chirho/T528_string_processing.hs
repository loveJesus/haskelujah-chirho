-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: hello world\n11\nhello-world
module Main where
myIntercalate :: String -> [String] -> String
myIntercalate _ [] = ""
myIntercalate _ [x] = x
myIntercalate sep (x:xs) = x ++ sep ++ myIntercalate sep xs
myLength :: [a] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
strLength :: String -> Int
strLength s = myLength s
main :: IO ()
main = do
  putStrLn (myIntercalate " " ["hello", "world"])
  print (strLength "hello world")
  putStrLn (myIntercalate "-" ["hello", "world"])
