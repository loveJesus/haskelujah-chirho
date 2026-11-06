-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UndecidableSuperClasses, FlexibleContexts #-}
module Main where

class CycleChirho [a] => CycleChirho a

needsListListChirho :: CycleChirho [[a]] => a -> String
needsListListChirho _ = "list list"

secondEdgeChirho :: CycleChirho a => a -> String
secondEdgeChirho valueChirho = needsListListChirho valueChirho

main :: IO ()
main = putStrLn "second edge"
