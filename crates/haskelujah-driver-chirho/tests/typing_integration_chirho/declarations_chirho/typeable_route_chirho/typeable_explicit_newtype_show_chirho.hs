-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, DeriveDataTypeable #-}
module Main where

import Data.Typeable (Typeable)

newtype AgeChirho = AgeChirho Int
  deriving stock (Show)
  deriving newtype (Typeable)

main :: IO ()
main = print (AgeChirho 5)
