-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilyDependencies #-}
module Main (main) where

type family FlipChirho aChirho = resultChirho | resultChirho -> aChirho where
  FlipChirho Int = Char
  FlipChirho Char = Int
  FlipChirho aChirho = aChirho

flipChirho :: FlipChirho aChirho -> FlipChirho aChirho
flipChirho valueChirho = valueChirho

main :: IO ()
main = do
  print (flipChirho 'c' :: Char)
  print (flipChirho 1.0 :: Double)
  print (flipChirho () :: ())
