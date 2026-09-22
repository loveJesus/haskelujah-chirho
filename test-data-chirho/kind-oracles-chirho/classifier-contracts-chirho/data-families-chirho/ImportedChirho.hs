-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module Main where
import qualified ProviderChirho as PChirho
type family StripChirho aChirho
type instance StripChirho (PChirho.DataChirho aChirho) = aChirho
type instance StripChirho (PChirho.AssociatedDataChirho aChirho) = aChirho
keepChirho :: StripChirho (PChirho.DataChirho Int) -> Int
keepChirho valueChirho = valueChirho
main :: IO ()
main = print (keepChirho 7)
