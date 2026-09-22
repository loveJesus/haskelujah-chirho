-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module Main where
import Data.Kind (Type)
data family FirstChirho aChirho
data family SecondChirho :: Type -> Type
type family StripChirho aChirho
type instance StripChirho (FirstChirho aChirho) = aChirho
type instance StripChirho (SecondChirho aChirho) = aChirho
firstChirho :: StripChirho (FirstChirho Int) -> Int
firstChirho valueChirho = valueChirho
secondChirho :: StripChirho (SecondChirho Int) -> Int
secondChirho valueChirho = valueChirho
main :: IO ()
main = print (firstChirho (secondChirho 7))
