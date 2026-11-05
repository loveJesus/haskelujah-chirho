-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ImpredicativeTypes, TypeApplications #-}
module Main where
type family IdChirho aChirho where IdChirho aChirho = aChirho
fChirho :: [forall aChirho. Maybe aChirho]
fChirho = undefined
myLengthChirho :: forall aChirho. [IdChirho aChirho] -> Int
myLengthChirho = undefined
gChirho :: [forall aChirho. Maybe aChirho] -> Int
gChirho xChirho = myLengthChirho @([forall aChirho. Maybe aChirho]) [xChirho, fChirho]
main :: IO ()
main = pure ()
