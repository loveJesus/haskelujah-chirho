-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, DerivingStrategies, MultiParamTypeClasses, FlexibleInstances, FlexibleContexts #-}
module Main where

class Show m => ShowLastChirho r m

instance ShowLastChirho Bool Int

newtype BoxChirho a = BoxChirho a
  deriving newtype (Show, ShowLastChirho Bool)

describeChirho :: ShowLastChirho r m => r -> m -> String
describeChirho _ lastChirho = show lastChirho

main :: IO ()
main = putStrLn (describeChirho True (BoxChirho (5 :: Int)))
