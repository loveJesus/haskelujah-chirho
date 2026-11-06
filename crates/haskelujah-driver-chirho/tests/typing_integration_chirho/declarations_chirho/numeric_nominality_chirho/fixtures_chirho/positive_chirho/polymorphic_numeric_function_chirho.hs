-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module PolymorphicNumericFunctionChirho where

plusOneChirho valueChirho = valueChirho + 1

integerChirho :: Int
integerChirho = plusOneChirho (7 :: Int)

floatingChirho :: Double
floatingChirho = plusOneChirho (7.5 :: Double)
