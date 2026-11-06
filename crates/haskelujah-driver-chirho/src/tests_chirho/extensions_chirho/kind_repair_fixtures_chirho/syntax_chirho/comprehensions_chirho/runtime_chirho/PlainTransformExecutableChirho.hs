-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TransformListComp #-}
module Main where
valueChirho :: [Int]
valueChirho = [ xChirho | xChirho <- ([1,2,3] :: [Int]), then reverse ]
main :: IO ()
main = print valueChirho
