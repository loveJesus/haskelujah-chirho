-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances #-}
module Main where
class CChirho a where
  type FChirho a
  type FChirho x = Maybe x
instance CChirho Int
valueChirho :: FChirho Int
valueChirho = True
main :: IO ()
main = pure ()
