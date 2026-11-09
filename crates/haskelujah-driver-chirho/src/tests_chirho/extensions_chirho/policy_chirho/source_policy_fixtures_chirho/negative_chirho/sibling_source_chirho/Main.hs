-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021 #-}
module Main where
import SiblingForallChirho (identityChirho)

main :: IO ()
main = print (identityChirho (7 :: Int))
