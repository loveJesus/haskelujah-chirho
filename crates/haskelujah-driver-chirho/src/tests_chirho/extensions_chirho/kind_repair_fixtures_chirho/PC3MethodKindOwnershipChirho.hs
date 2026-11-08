-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures #-}
module Main where
import Data.Kind (Type)
data TagChirho (a :: k) = TagChirho
class NameChirho (f :: k) where
  nameChirho :: TagChirho f -> String
unaryNameChirho :: TagChirho (f :: Type -> Type) -> String
unaryNameChirho _ = "unary"
instance NameChirho Maybe where
  nameChirho t = unaryNameChirho t
main :: IO ()
main = putStrLn (nameChirho (TagChirho :: TagChirho Maybe))
