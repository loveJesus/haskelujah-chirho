-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, PolyKinds, KindSignatures #-}
module Main where

data EqualChirho (leftChirho :: kindChirho) (rightChirho :: kindChirho) where
  ReflChirho :: EqualChirho valueChirho valueChirho

castChirho :: EqualChirho leftChirho rightChirho -> leftChirho -> rightChirho
castChirho ReflChirho valueChirho = valueChirho

main :: IO ()
main = print (castChirho ReflChirho (42 :: Int))
