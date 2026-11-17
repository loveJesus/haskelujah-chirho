-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module Main where

data InnerChirho a_chirho = InnerChirho a_chirho
data OuterChirho a_chirho = OuterChirho (InnerChirho a_chirho)
class CChirho a_chirho where
  mChirho :: OuterChirho a_chirho -> Integer
instance CChirho Integer where
  mChirho _ = 3

main :: IO ()
main = print (mChirho (OuterChirho (InnerChirho (5 :: Integer))))
