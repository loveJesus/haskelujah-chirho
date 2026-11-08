-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures, TypeFamilies #-}
module Main where
class CChirho (aChirho :: kChirho) where
  type FChirho aChirho
  type FChirho xChirho = Maybe xChirho
main :: IO ()
main = pure ()
