-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleInstances #-}
module Main where

import Prelude hiding (Eq, (==))

-- The program's own class, spelled like the Prelude's.
class Eq a where
  (==) :: a -> a -> Bool

-- A program class whose superclass is the compiler's Ord.
class Ord a => RankChirho a where
  rankChirho :: a -> Int

instance RankChirho [Double] where
  rankChirho = length

useRankChirho :: RankChirho a => a -> Int
useRankChirho valueChirho = rankChirho valueChirho

-- The use demands only `RankChirho [Double]`, whose Ord superclass field is the compiler's
-- `Ord [Double]` dictionary.
main :: IO ()
main = print (useRankChirho [1.5, 2.5 :: Double])
