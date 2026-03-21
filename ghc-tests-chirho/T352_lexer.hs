-- TEST: compile_and_run
-- EXPECTED: NUM(3)\nPLUS\nNUM(2)
module Main where
data Token = TNum Int | TPlus | TMinus | TStar | TEnd
lexOne :: String -> (Token, String)
lexOne (c:rest) | c == 43 = (TPlus, rest) | c == 45 = (TMinus, rest) | c == 42 = (TStar, rest) | c >= 48 = (TNum (c - 48), rest) | otherwise = (TEnd, rest)
lexOne [] = (TEnd, [])
showToken :: Token -> String
showToken (TNum n) = "NUM(" ++ show n ++ ")"; showToken TPlus = "PLUS"; showToken TMinus = "MINUS"; showToken TStar = "STAR"; showToken TEnd = "END"
main :: IO ()
main = do
  let (t1, r1) = lexOne [51, 43, 50]
  let (t2, r2) = lexOne r1
  let (t3, _) = lexOne r2
  putStrLn (showToken t1)
  putStrLn (showToken t2)
  putStrLn (showToken t3)
