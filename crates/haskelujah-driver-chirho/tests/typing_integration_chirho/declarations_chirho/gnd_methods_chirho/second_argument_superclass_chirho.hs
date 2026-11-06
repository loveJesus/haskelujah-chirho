-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances #-}
module Main where

class Show b => ShowSecondChirho a b

instance ShowSecondChirho Int Bool

describeChirho :: ShowSecondChirho a b => a -> b -> String
describeChirho _ secondChirho = show secondChirho

main :: IO ()
main = putStrLn (describeChirho (5 :: Int) True)
