-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ImpredicativeTypes #-}
module Main where
valuesChirho :: [forall aChirho. aChirho -> aChirho]
valuesChirho = [id]
reversedChirho :: [forall aChirho. aChirho -> aChirho]
reversedChirho = reverse valuesChirho
main :: IO ()
main = pure ()
