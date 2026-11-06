-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
module Main where

class DescribeChirho aChirho where
  describeChirho :: aChirho -> String

instance DescribeChirho Int where
  describeChirho _ = "integer"

instance DescribeChirho Bool where
  describeChirho _ = "boolean"

newtype BoxChirho aChirho = BoxChirho aChirho
  deriving newtype (Show, DescribeChirho)

main :: IO ()
main = do
  putStrLn (show (BoxChirho (7 :: Int)))
  putStrLn (show (BoxChirho True))
  putStrLn (describeChirho (BoxChirho (7 :: Int)))
  putStrLn (describeChirho (BoxChirho True))
