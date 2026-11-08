-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TransformListComp, ParallelListComp #-}
module ParallelKeyEscapeChirho where
import GHC.Exts (groupWith)
valueChirho = [ (xChirho,yChirho) | xChirho <- ([1,2] :: [Int]), then group by yChirho using groupWith | yChirho <- ([3,4] :: [Int]) ]
