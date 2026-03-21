-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

data ConversionReportChirho = MkConversionReportChirho
  Int
  String
  String
  String
  Int
  Int

decimalValueChirho :: ConversionReportChirho -> Int
decimalValueChirho (MkConversionReportChirho valueChirho _ _ _ _ _) = valueChirho

binaryValueChirho :: ConversionReportChirho -> String
binaryValueChirho (MkConversionReportChirho _ binaryChirho _ _ _ _) = binaryChirho

octalValueChirho :: ConversionReportChirho -> String
octalValueChirho (MkConversionReportChirho _ _ octalChirho _ _ _) = octalChirho

hexValueChirho :: ConversionReportChirho -> String
hexValueChirho (MkConversionReportChirho _ _ _ hexChirho _ _) = hexChirho

digitSumChirho :: ConversionReportChirho -> Int
digitSumChirho (MkConversionReportChirho _ _ _ _ digitSumValueChirho _) = digitSumValueChirho

bitCountChirho :: ConversionReportChirho -> Int
bitCountChirho (MkConversionReportChirho _ _ _ _ _ bitCountValueChirho) = bitCountValueChirho

absIntChirho :: Int -> Int
absIntChirho valueChirho =
  if valueChirho < 0 then 0 - valueChirho else valueChirho

digitTextChirho :: Int -> String
digitTextChirho 0 = "0"
digitTextChirho 1 = "1"
digitTextChirho 2 = "2"
digitTextChirho 3 = "3"
digitTextChirho 4 = "4"
digitTextChirho 5 = "5"
digitTextChirho 6 = "6"
digitTextChirho 7 = "7"
digitTextChirho 8 = "8"
digitTextChirho 9 = "9"
digitTextChirho 10 = "A"
digitTextChirho 11 = "B"
digitTextChirho 12 = "C"
digitTextChirho 13 = "D"
digitTextChirho 14 = "E"
digitTextChirho 15 = "F"
digitTextChirho _ = "?"

toBasePositiveChirho :: Int -> Int -> String
toBasePositiveChirho _ 0 = "0"
toBasePositiveChirho baseChirho valueChirho =
  if valueChirho < baseChirho
  then digitTextChirho valueChirho
  else toBasePositiveChirho baseChirho (div valueChirho baseChirho)
       ++ digitTextChirho (mod valueChirho baseChirho)

formatBaseChirho :: String -> Int -> Int -> String
formatBaseChirho prefixChirho baseChirho valueChirho =
  if valueChirho < 0
  then "-" ++ prefixChirho ++ toBasePositiveChirho baseChirho (absIntChirho valueChirho)
  else prefixChirho ++ toBasePositiveChirho baseChirho valueChirho

decimalDigitSumChirho :: Int -> Int
decimalDigitSumChirho valueChirho =
  let positiveChirho = absIntChirho valueChirho in
  if positiveChirho < 10
  then positiveChirho
  else mod positiveChirho 10 + decimalDigitSumChirho (div positiveChirho 10)

binaryBitCountChirho :: Int -> Int
binaryBitCountChirho valueChirho =
  let positiveChirho = absIntChirho valueChirho in
  if positiveChirho == 0
  then 0
  else mod positiveChirho 2 + binaryBitCountChirho (div positiveChirho 2)

buildReportChirho :: Int -> ConversionReportChirho
buildReportChirho valueChirho =
  MkConversionReportChirho
    valueChirho
    (formatBaseChirho "" 2 valueChirho)
    (formatBaseChirho "" 8 valueChirho)
    (formatBaseChirho "0x" 16 valueChirho)
    (decimalDigitSumChirho valueChirho)
    (binaryBitCountChirho valueChirho)

renderReportChirho :: ConversionReportChirho -> IO ()
renderReportChirho reportChirho = do
  putStrLn ("decimal   : " ++ show (decimalValueChirho reportChirho))
  putStrLn ("binary    : " ++ binaryValueChirho reportChirho)
  putStrLn ("octal     : " ++ octalValueChirho reportChirho)
  putStrLn ("hex       : " ++ hexValueChirho reportChirho)
  putStrLn ("digit sum : " ++ show (digitSumChirho reportChirho))
  putStrLn ("bit count : " ++ show (bitCountChirho reportChirho))

main :: IO ()
main = do
  putStrLn "=== Haskelujah Number Converter Chirho ==="
  putStrLn "Enter an integer:"
  inputChirho <- getLine
  let valueChirho = ((read inputChirho) :: Int)
  let reportChirho = buildReportChirho valueChirho
  putStrLn ""
  renderReportChirho reportChirho
  putStrLn ""
  putStrLn "Single-binary Haskell scripting, by the grace of God."
