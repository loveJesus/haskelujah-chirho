-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
module Main where
newtype BoxChirho aChirho = BoxChirho aChirho deriving newtype Show
main :: IO ()
main = print (BoxChirho (7 :: Int))
