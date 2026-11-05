-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ImpredicativeTypes #-}
module Main where
fChirho :: [forall aChirho. Maybe aChirho]
fChirho = undefined
ordinaryLengthChirho :: [aChirho] -> Int
ordinaryLengthChirho = undefined
gChirho :: [forall aChirho. Maybe aChirho] -> Int
gChirho xChirho = ordinaryLengthChirho [xChirho, fChirho]
main :: IO ()
main = pure ()
