-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UndecidableSuperClasses, FlexibleContexts #-}
module Main where

class CycleChirho [a] => CycleChirho a

needsIntChirho :: CycleChirho Int => String
needsIntChirho = "int"

unreachedChirho :: CycleChirho a => a -> String
unreachedChirho _ = needsIntChirho

main :: IO ()
main = putStrLn "unreached"
