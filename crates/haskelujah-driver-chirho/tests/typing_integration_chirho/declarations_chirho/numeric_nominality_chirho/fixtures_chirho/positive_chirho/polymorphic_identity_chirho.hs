-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module PolymorphicIdentityChirho where

identityChirho valueChirho = valueChirho

integerChirho :: Int
integerChirho = identityChirho (7 :: Int)

floatingChirho :: Double
floatingChirho = identityChirho (7.5 :: Double)
