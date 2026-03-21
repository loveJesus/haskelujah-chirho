-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Just 123\nNothing\nJust [1,2,3]
module Main where

data ParseResult a = ParseOk a String | ParseFail

isDigit :: Char -> Bool
isDigit c = c >= '0' && c <= '9'

charToInt :: Char -> Int
charToInt c
  | c == '0' = 0
  | c == '1' = 1
  | c == '2' = 2
  | c == '3' = 3
  | c == '4' = 4
  | c == '5' = 5
  | c == '6' = 6
  | c == '7' = 7
  | c == '8' = 8
  | c == '9' = 9
  | otherwise = 0

parseDigit :: String -> ParseResult Int
parseDigit [] = ParseFail
parseDigit (c:cs) = if isDigit c then ParseOk (charToInt c) cs else ParseFail

parseNumGo :: Int -> Int -> String -> ParseResult Int
parseNumGo count acc s = case parseDigit s of
  ParseOk d rest -> parseNumGo (count + 1) (acc * 10 + d) rest
  ParseFail -> if count == 0 then ParseFail else ParseOk acc s

parseNumber :: String -> ParseResult Int
parseNumber = parseNumGo 0 0

parseManyDigits :: [Int] -> String -> ([Int], String)
parseManyDigits acc s = case parseDigit s of
  ParseFail -> (myReverse acc, s)
  ParseOk v rest -> parseManyDigits (v : acc) rest

myReverse :: [a] -> [a]
myReverse xs = revGo [] xs
  where revGo a [] = a
        revGo a (y:ys) = revGo (y:a) ys

showMaybeInt :: ParseResult Int -> String
showMaybeInt ParseFail = "Nothing"
showMaybeInt (ParseOk n _) = "Just " ++ show n

main :: IO ()
main = do
  putStrLn (showMaybeInt (parseNumber "123abc"))
  putStrLn (showMaybeInt (parseNumber "abc"))
  let (digits, _) = parseManyDigits [] "123end"
  putStrLn ("Just " ++ show digits)
