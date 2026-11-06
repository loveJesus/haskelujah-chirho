-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, MultiParamTypeClasses, FlexibleInstances, FlexibleContexts #-}
module Main where

class Show m => ShowLastChirho r m

instance ShowLastChirho Bool Int

newtype AgeChirho = AgeChirho Int
  deriving (Show, ShowLastChirho Bool)

describeChirho :: ShowLastChirho r m => r -> m -> String
describeChirho _ lastChirho = show lastChirho

main :: IO ()
main = putStrLn (describeChirho True (AgeChirho 5))
