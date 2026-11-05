-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, LiberalTypeSynonyms, ImpredicativeTypes #-}
module Main where
type family FChirho aChirho
type instance FChirho [aChirho] = Maybe aChirho
type family IdChirho aChirho
type instance IdChirho aChirho = aChirho
fChirho :: [forall aChirho. FChirho [aChirho]]
fChirho = undefined
myLengthChirho :: [IdChirho aChirho] -> Int
myLengthChirho = undefined
hChirho xChirho = myLengthChirho xChirho
gChirho :: [forall aChirho. Maybe aChirho] -> Int
gChirho xChirho = hChirho [xChirho, fChirho]
main :: IO ()
main = pure ()
