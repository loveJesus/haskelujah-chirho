-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleContexts, UndecidableInstances #-}
module Main where
newtype WrapChirho = WrapChirho Int
instance Show Int => Show WrapChirho where
  show _ = "user"
main :: IO ()
main = putStrLn (show (WrapChirho 5))
