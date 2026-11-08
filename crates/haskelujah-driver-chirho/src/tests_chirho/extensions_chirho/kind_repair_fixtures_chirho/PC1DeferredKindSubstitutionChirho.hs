-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures #-}
module Main where
import Data.Kind (Type)
data TagChirho (a :: k) = TagChirho
class NameChirho (f :: k) where
  nameChirho :: TagChirho f -> String
instance NameChirho Maybe where
  nameChirho _ = "Maybe"
kindFixChirho :: TagChirho (f :: Type -> Type) -> TagChirho f
kindFixChirho t = t
describeChirho t = nameChirho (kindFixChirho t)
main :: IO ()
main = putStrLn (describeChirho (TagChirho :: TagChirho Maybe))
