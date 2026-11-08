-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ImpredicativeTypes #-}
module Main where
type family IdChirho aChirho where IdChirho aChirho = aChirho
valuesChirho :: [[forall aChirho. Maybe aChirho]]
valuesChirho = undefined
myLengthChirho :: [IdChirho aChirho] -> Int
myLengthChirho = undefined
gChirho :: Int
gChirho = myLengthChirho valuesChirho
main :: IO ()
main = pure ()
