-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module Main where

class Typeable a where
  describeChirho :: a -> String
  describeChirho _ = "default"

instance Typeable Int where
  describeChirho _ = "int"

newtype AgeChirho = AgeChirho Int deriving (Typeable)

main :: IO ()
main = putStrLn (describeChirho (AgeChirho 5))
