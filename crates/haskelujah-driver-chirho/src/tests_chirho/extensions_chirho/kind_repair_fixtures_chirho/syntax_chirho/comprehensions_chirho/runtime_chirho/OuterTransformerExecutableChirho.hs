-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TransformListComp #-}
module Main where
import Data.List (inits)
transformerChirho :: [aChirho] -> [[aChirho]]
transformerChirho = inits
valueChirho :: [[Int]]
valueChirho = [ transformerChirho | transformerChirho <- ([1,2] :: [Int]), then group using transformerChirho ]
main :: IO ()
main = print valueChirho
