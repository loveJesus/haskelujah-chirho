-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures, ScopedTypeVariables #-}
module Main where
data TagChirho (a :: k) = TagChirho
data PairChirho (f :: k) (g :: k) = PairChirho
class NameChirho (f :: k) where
  nameChirho :: TagChirho f -> String
instance NameChirho Maybe where
  nameChirho _ = "Maybe"
instance (NameChirho f, NameChirho g) => NameChirho (PairChirho f g) where
  nameChirho _ = "Pair"
pairNameChirho :: forall k (f :: k) (g :: k). (NameChirho f, NameChirho g) => TagChirho f -> TagChirho g -> String
pairNameChirho _ _ = nameChirho (TagChirho :: TagChirho (PairChirho f g))
main :: IO ()
main = putStrLn (pairNameChirho (TagChirho :: TagChirho Maybe) (TagChirho :: TagChirho Maybe))
