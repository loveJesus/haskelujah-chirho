-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
module Main where
newtype AgeChirho = AgeChirho Int deriving newtype (Show, Num)
data PersonChirho = PersonChirho String AgeChirho deriving Show
newtype OlderChirho = OlderChirho AgeChirho deriving newtype Show
main :: IO ()
main = do
  print (PersonChirho "Ann" (AgeChirho 7))
  print (OlderChirho (AgeChirho 8))
  case PersonChirho "Bo" (AgeChirho 5 + 1) of
    PersonChirho _ (AgeChirho nChirho) -> print nChirho
