-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, StandaloneDeriving #-}
module Main where
newtype AgeChirho = AgeChirho Int
deriving newtype instance Show AgeChirho
main :: IO ()
main = print (AgeChirho 7)
