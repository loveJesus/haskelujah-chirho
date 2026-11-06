-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UndecidableSuperClasses, FlexibleContexts #-}
module Main where

class CycleChirho [a] => CycleChirho a

needsListChirho :: CycleChirho [a] => a -> String
needsListChirho _ = "list"

firstEdgeChirho :: CycleChirho a => a -> String
firstEdgeChirho valueChirho = needsListChirho valueChirho

main :: IO ()
main = putStrLn "first edge"
